use tokio_stream::{self as stream, StreamExt};

// https://docs.rs/tokio/latest/tokio/macro.select.html
/*
The complete lifecycle of a select! expression is as follows:
Evaluate all provided <precondition> expressions. If the precondition returns false, disable the branch for the remainder of the current call to select!. Re-entering select! due to a loop clears the “disabled” state.
Aggregate the <async expression>s from each branch, including the disabled ones. If the branch is disabled, <async expression> is still evaluated, but the resulting future is not polled.
Concurrently await on the results for all remaining <async expression>s.
Once an <async expression> returns a value, attempt to apply the value to the provided <pattern>, if the pattern matches, evaluate <handler> and return. If the pattern does not match, disable the current branch and for the remainder of the current call to select!. Continue from step 3.
If all branches are disabled, evaluate the else expression. If no else branch is provided, panic.
 */
async fn f1() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let next = tokio::select! {
        v = stream1.next() => v.unwrap(),
        v = stream2.next() => v.unwrap(),
    };

    dbg!(&next);

    assert!(next == 1 || next == 4);
}

async fn f2() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let mut values = vec![];

    loop {
        tokio::select! {
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            else => break,
        }
    }

    values.sort();
    assert_eq!(&[1, 2, 3, 4, 5, 6], &values[..]);
}

#[tokio::test]
async fn test1() {
    f1().await;
    f2().await;
}
