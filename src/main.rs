use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};
use warp::Filter;

#[derive(Deserialize)]
struct RequestBody {
    unique_id: String,
}

#[tokio::main]
async fn main() {
    // Shared state between handlers
    let shared_state = Arc::new(Mutex::new(HashMap::new()));

    // Clone the state to move into the warp filter
    let shared_state_filter = warp::any().map(move || Arc::clone(&shared_state));

    let wait_for_second_party = warp::post()
        .and(warp::path("wait-for-second-party"))
        .and(warp::path::param::<String>())
        .and(shared_state_filter.clone())
        .and_then(handle_request);

    warp::serve(wait_for_second_party)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_request(
    unique_id: String,
    shared_state: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Create a one-shot channel
    let (sender, receiver) = oneshot::channel();

    // Acquire the lock and check if there's a waiting party
    let mut state = shared_state.lock().await;

    if let Some(existing_sender) = state.remove(&unique_id) {
        // Notify the waiting party
        let _ = existing_sender.send(());
        Ok(warp::reply::json(&"Second party arrived"))
    } else {
        // No waiting party, so insert this sender into the state
        state.insert(unique_id.clone(), sender);

        // Release the lock before awaiting
        drop(state);

        // Wait for either the second party or the timeout
        let result = tokio::select! {
            _ = receiver => {
                Ok(warp::reply::json(&"Sync complete"))
            },
            _ = sleep(Duration::from_secs(10)) => {
                // Acquire the lock again to clean up
                let mut state = shared_state.lock().await;
                state.remove(&unique_id);
                Err(warp::reject::custom(TimeoutError))
            }
        };

        result
    }
}

#[derive(Debug)]
struct TimeoutError;

impl warp::reject::Reject for TimeoutError {}
