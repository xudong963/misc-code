/// Async code should never spend a long time without reaching an .await.
/*
use std::{thread, time};

fn large_num_threads() {
    let num = 100000000;
    let mut thread_join_handles = Vec::with_capacity(num);
    for _index in 0..num {
        thread_join_handles.push(thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(2));
        }))
    }
    for thread_join_handle in thread_join_handles {
        let _res = thread_join_handle.join();
    }
}
 */

/// An example about blocking without await
use std::time::Duration;

async fn sleep_then_print1(timer: i32) {
    println!("Start timer {}.", timer);

    // No .await here!
    std::thread::sleep(Duration::from_secs(1));

    println!("Timer {} done.", timer);
}

#[tokio::test]
async fn test1() {
    // The join! macro lets you run multiple things concurrently.
    tokio::join!(
        sleep_then_print1(1),
        sleep_then_print1(2),
        sleep_then_print1(3),
    );
}

/// use await to fix blocking
async fn sleep_then_print2(timer: i32) {
    println!("Start timer {}.", timer);

    tokio::time::sleep(Duration::from_secs(1)).await;

    println!("Timer {} done.", timer);
}

#[tokio::test]
async fn test2() {
    // The join! macro lets you run multiple things concurrently.
    tokio::join!(
        sleep_then_print2(1),
        sleep_then_print2(2),
        sleep_then_print2(3),
    );
}

/// Sometimes we just want to block the thread. Due to
/// 1. Expensive CPU-bound computation.
/// 2. Synchronous IO.
/// Core: move the blocking operator out of tokio thread pool
/// 1. Use the tokio::task::spawn_blocking function.
/// 2. Use the rayon crate.
/// 3. Spawn a dedicated thread with std::thread::spawn.

/// 1. The Tokio runtime includes a separate thread pool specifically for running blocking functions
/// This thread pool has an upper limit of around 500 threads.
/// Since the thread pool has so many threads, it is best suited for blocking IO such as interacting with the file system
/// The thread pool is poorly suited for expensive CPU-bound computations, since it has many more threads than you have CPU cores
#[tokio::test]
async fn test3() {
    // This is running on Tokio. We may not block here.
    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a thread where blocking is fine.
        println!("Inside spawn_blocking");
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}

/// 2. Use rayon to process expensive CPU-bound tasks
async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (send, recv) = tokio::sync::oneshot::channel();

    // Spawn a task on rayon.
    rayon::spawn(move || {
        // Perform an expensive computation.
        // Only use one thread of rayon thread pool
        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        // Send the result back to Tokio.
        let _ = send.send(sum);
    });

    // Wait for the rayon task.
    recv.await.expect("Panic in rayon::spawn")
}

#[tokio::test]
async fn test4() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
