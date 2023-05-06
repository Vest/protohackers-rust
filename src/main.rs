use std::env;
use std::error::Error;
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::TcpListenerStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:23232".to_string());

    let listener = TcpListener::bind(&addr).await?;

    let _ = tokio::spawn(async move {
        let mut incoming = TcpListenerStream::new(listener);

        while let Some(mut stream) = incoming.next().await.transpose().unwrap() {
            let (mut r, mut w) = stream.split();

            println!("copied {} bytes", tokio::io::copy(&mut r, &mut w).await.unwrap());
        }
    }).await;

    Ok(())
}
