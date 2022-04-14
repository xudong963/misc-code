use crossbeam;
use std::thread;
use std::time::Duration;
/*
fn play_std_thread() {
    let str1 = vec!["a", "b", "c"];
    let str2 = vec!["d", "e", "f"];

    let mut iter = str1.iter().zip(str2);
    /// iterator isn't safe shared in multi threads
    /// pass a reference to a stack-allocated variable to one of these threads,
    /// there's no guarantee that the variable will still be valid by the time the thread executes.
    let join_handle = thread::spawn(|| {
        while let Some((a, b)) = iter.next() {
            println!("{}-{}", a, b);
        }
    });
    let _ = join_handle.join();
}
 */

/// use scope thread
/// that are guaranteed to exit before a certain scope ends.
fn play_scope_thread() {
    let str1 = vec!["a", "b", "c"];
    let str2 = vec!["d", "e", "f"];

    let mut iter = str1.iter().zip(str2);
    crossbeam::scope(|scope| {
        while let Some((a, b)) = iter.next() {
            scope.spawn(move |_| {
                println!("{}-{}", a, b);
            });
        }
    })
    .unwrap();
}

#[cfg(test)]
mod tests {
    // use crate::play_std_thread;
    use crate::play_scope_thread;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        // play_std_thread();
        play_scope_thread()
    }
}
