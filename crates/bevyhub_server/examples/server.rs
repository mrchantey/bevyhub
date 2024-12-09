#[tokio::main]
#[rustfmt::skip]
pub async fn main() -> anyhow::Result<()> {
	bevyhub_server::server::Server::default().run().await
}
