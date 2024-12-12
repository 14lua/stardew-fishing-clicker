mod clicker;

#[tokio::main]
async fn main() {
    clicker::listen_key().await;
}
