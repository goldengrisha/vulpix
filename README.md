# vulpix
This project is a small web service implemented in Rust using the `tokio` runtime and `warp` web framework. The service provides a single endpoint that allows two parties to synchronize. When one party makes a POST request, the response will be delayed until the second party makes a request to the same URL or a timeout occurs (10 seconds).


## Installation

1. **Clone the repository**:
    ```sh
    git clone git@github.com:goldengrisha/vulpix.git
    cd rust-sync-web-service
    ```

2. **Build the project**:
    ```sh
    cargo build
    ```

## Running the Service

Start the server by running:

```sh
cargo run
```

The server will start and listen on `127.0.0.1:3030`.

## Endpoint

### `/wait-for-second-party/:unique-id`

**Method**: `POST`

This endpoint allows two parties to synchronize using a unique identifier.

#### Example Usage

1. Open two terminal windows.

2. In the first terminal, run:
    ```sh
    curl -X POST http://127.0.0.1:3030/wait-for-second-party/unique-id-123
    ```
    This request will hang until the second party arrives or the timeout occurs.

3. In the second terminal, run:
    ```sh
    curl -X POST http://127.0.0.1:3030/wait-for-second-party/unique-id-123
    ```
    Both requests should now complete successfully with the message `{"message":"Second party arrived"}`.

4. If you don't make the second request within 10 seconds, the first request will timeout and return a timeout error:
    ```sh
    curl -X POST http://127.0.0.1:3030/wait-for-second-party/unique-id-123
    {"error":"Timeout"}
    ```

### Handling Responses

You can use tools like `jq` to format the JSON response for better readability:

```sh
curl -X POST http://127.0.0.1:3030/wait-for-second-party/unique-id-123 | jq
```

## Error Handling

If the second party does not arrive within 10 seconds, the first request will timeout, and you will receive a timeout error in the response.
