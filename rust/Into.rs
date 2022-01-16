/// A value-to-value conversion that consumes the input value. The opposite of From
/// From<T> for U implies Into<U> for T



/// `String` implements `Into<Vec<u8>>`
/// In order to express that we want a generic function to take all arguments that can be converted to a specified type `T`, 
/// we can use a trait bound of `Into<T>`.
fn is_hello<T: Into<Vec<u8>>>(s: T) {
    let bytes = b"hello".to_vec();
    assert_eq!(bytes, s.into());
}

fn main() {
    let s = "hello".to_string();
    is_hello(s);
}