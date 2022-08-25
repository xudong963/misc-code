use async_channel::{unbounded, RecvError};

async fn foo() {
    let (s, r) = unbounded();

    assert_eq!(s.send(1).await, Ok(()));
    assert_eq!(s.send(2).await, Ok(()));
    drop(s);

    assert_eq!(r.recv().await, Ok(1));
    assert_eq!(r.recv().await, Ok(2));
}

async fn foo1() {
    let (s, r) = async_channel::bounded(2);

    assert_eq!(s.send(1).await, Ok(()));
    assert_eq!(s.send(2).await, Ok(()));
    drop(s);

    assert_eq!(r.recv().await, Ok(1));
    assert_eq!(r.recv().await, Ok(2));
}



#[tokio::test]
async fn it_works() {
    foo().await;
    foo1().await
}
