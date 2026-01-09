use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::{future::Future, panic::catch_unwind, thread};

use async_task::{Runnable, Task};
use futures_lite::future;
use std::sync::LazyLock;

fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                let _ = catch_unwind(|| runnable.run());
            }
        });
        tx
    });

    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    task
}

fn main() {}
