use async_std::io;
use async_std::task;
use async_std::future;
use async_std::task::{Context, Poll};

fn poll_greeting(_: &mut Context<'_>) -> Poll<String> {
    Poll::Ready("hello world".to_string())
}

async fn doit() -> io::Result<()> {
    println!("{}", future::poll_fn(poll_greeting).await);

    Ok(())
}

fn main() {
    task::block_on(doit());
}
