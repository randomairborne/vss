# vss
### valkyrie_pilot's `shutdown_signal`

This is a very simple crate, made to be used with
axum's `graceful_shutdown` method, like so:

```rust
#[tokio::main]
async fn main() {
axum::serve(tcp, app)
    .with_graceful_shutdown(vss::shutdown_signal())
    .await
    .unwrap();
}
```