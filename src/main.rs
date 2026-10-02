use std::{future::{Future, poll_fn}, pin::Pin, task::{Context, Poll}};
use futures::channel::oneshot;

struct MyCode {}

impl Future for MyCode {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }
}

async fn sum(a: u64, b: u64) -> u64 {
    let mc = MyCode{}.await;

    let mut n = 0;
    let xc = poll_fn(|cx| {
        n += 1;

        if n < 10 {
            cx.waker().wake_by_ref();

            Poll::Pending
        } else {
            Poll::Ready(n)
        }
    }).await;

    a + b + xc
}

async fn run() {
    let s = sum(3, 4).await;

    println!("{}", s);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(pool) = futures::executor::ThreadPool::new() {
        let (tx, rx) = oneshot::channel();

        pool.spawn_ok(async {
            run().await;

            tx.send(());
        });

        futures::executor::block_on(rx);
    }

    Ok(())
}
