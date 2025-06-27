
## 🛰️ gRPC Communication Flow in Rust with Tonic

In your Rust project using **gRPC** and **Tonic**, the communication model is simple:

> **The client initiates a request**, and **the server responds**.

This is a fundamental principle of **gRPC (Remote Procedure Call)**. Below is a concise explanation with a **text-based diagram** based on your `comand.proto`, `main.rs` (server), and `clients.rs` (client).

---

### 🚦 1. Communication Flow Explained

In your gRPC setup:

* The **client** (`clients.rs`) sends a `CommandRequest` to the server by calling a method defined in the service (`ExecuteCommand`).
* The **server** (`main.rs`) receives the request, executes the logic in `MyComandService::execute_command`, and replies with a `CommandResponse`.
* The communication is **unary and client-initiated** — meaning the server only acts when a client sends a request.

#### Key Details:

* The client sends a `CommandRequest` with a field like: `command: "test_command"`.
* The server processes it and returns: `output: "Command executed: test_command"`.
* The server **does not send anything spontaneously** — it **waits** for client requests.

---

### 🖼️ 2. Text-Based Flow Diagram

```text
[Client: clients.rs]                    [Network: HTTP/2]                    [Server: main.rs]
       |                                       |                                     |
       | 1. Create CommandRequest              |                                     |
       |    { command: "test_command" }        |                                     |
       |------------------------------------->| (Serialized via Protocol Buffers)    |
       |                                       |------------------------------------->|
       |                                       |                                     | 2. Deserialize CommandRequest
       |                                       |                                     | 3. Execute execute_command()
       |                                       |                                     | 4. Create CommandResponse
       |                                       |                                     |    { output: "Command executed: test_command" }
       |                                       |<-------------------------------------| (Serialized Response)
       | 5. Deserialize CommandResponse                                        |
       | 6. Print: "Command executed: test_command"                          |
```

> ✅ **Right arrow (`----->`)**: client sends the request
> ✅ **Left arrow (`<-----`)**: server sends the response
> 🟡 The **client initiates**; the **server waits** and responds.

---

### 🧩 3. Code Details

#### 📍 Client (`clients.rs`)

```rust
let mut client = ComandServiceClient::connect("http://127.0.0.1:50051").await?;
let request = tonic::Request::new(CommandRequest {
    command: "test_command".into(),
});
let response = client.execute_command(request).await?;
println!("Response: {:?}", response.into_inner().output);
```

* Connects to the server
* Builds the request
* Calls the RPC method
* Awaits and prints the response

#### 📍 Server (`main.rs`)

```rust
#[tonic::async_trait]
impl ComandService for MyComandService {
    async fn execute_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let command = request.into_inner().command;
        let reply = CommandResponse {
            output: format!("Command executed: {}", command),
        };
        Ok(Response::new(reply))
    }
}
```

* Implements server logic
* Only responds **after** receiving a client request

#### 📍 Proto file (`comand.proto`)

```proto
service ComandService {
    rpc ExecuteCommand (CommandRequest) returns (CommandResponse);
}
```

* Defines a **unary RPC**: one request, one response.

---

### 🧪 4. How to Test

#### ✅ Step 1: Setup & Build

```bash
cargo clean
cargo build
```

Make sure `prost v0.13.5` is used:

```bash
cargo tree | grep prost
```

#### ✅ Step 2: Run the Server

```bash
cargo run --bin server
```

Output:

```
Server running on 127.0.0.1:50051
```

#### ✅ Step 3: Run the Client

```bash
cargo run --bin client
```

Expected output:

```
Response: "Command executed: test_command"
```

#### ✅ Step 4: Test with `grpcurl`

Install `grpcurl`:

```bash
sudo apt-get install grpcurl
```

Call the service:

```bash
grpcurl -plaintext -d '{"command": "test"}' 127.0.0.1:50051 comand.ComandService/ExecuteCommand
```

Expected response:

```json
{
  "output": "Command executed: test"
}
```

#### ✅ Step 5: Confirm the server does **nothing** without a client

Run the server **alone** → No output. The server waits passively.

#### ✅ Step 6: Handle errors (e.g., empty command)

Modify the server:

```rust
if command.is_empty() {
    return Err(Status::invalid_argument("Command cannot be empty"));
}
```

Test:

```bash
grpcurl -plaintext -d '{"command": ""}' 127.0.0.1:50051 comand.ComandService/ExecuteCommand
```

Expected error:

```
ERROR:
  Code: InvalidArgument
  Message: Command cannot be empty
```

---

### 🔁 5. What If the Server Sends First? (Not in this project)

gRPC **does support server-initiated communication** through:

* **Server Streaming RPC**
* **Bidirectional Streaming RPC**

Example `comand.proto` (not used here):

```proto
rpc ExecuteCommandStream (CommandRequest) returns (stream CommandResponse);
```

In your case, **only unary RPC** is used → the **client always starts**.

---

### 🛠️ 6. Troubleshooting Tips

| Issue                     | Solution                     |              |
| ------------------------- | ---------------------------- | ------------ |
| Server not running        | `cargo run --bin server`     |              |
| Port conflict             | \`netstat -tuln              | grep 50051\` |
| Wrong address             | Use `http://127.0.0.1:50051` |              |
| Multiple `prost` versions | \`cargo tree                 | grep prost\` |

---

### 🧾 7. Summary

* The **client** sends the request.
* The **server** waits and **responds**.
* This flow is confirmed through actual output, code structure, and tools like `grpcurl`.
* For advanced needs, gRPC offers streaming — but it's not used here.
