use std::io::Result;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
