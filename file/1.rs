use async_std::prelude::*;
use async_std::fs::File;
use async_std::task;
use async_std::io;

async fn create_and_write() -> io::Result<()> {
    let mut file = File::create("a.txt").await?;
    file.write_all(b"Hello world!").await?;

    Ok(())
}

fn main() {
    task::block_on(create_and_write());
}
