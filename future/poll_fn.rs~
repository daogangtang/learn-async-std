use std::time::Duration;
use async_std::future;
use async_std::io;
use async_std::task;

async fn doit() -> io::Result<()> {
    let dur = Duration::from_secs(1);
    let fut = future::pending();

    let res: io::Result<()> = io::timeout(dur, fut).await;
    println!("{:?}", res);

    Ok(())
}

fn main() {
    task::block_on(doit());
}
