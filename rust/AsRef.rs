
/// Used to do a cheap reference-to-reference conversion.


/// accept arguments of different types as long as they can be converted to the specified type T
/// `is_hello` is a generic function that takes an `AsRef<str>`, it wants to accept all references that can be 
/// converted to `&str` as an argument.

fn is_hello<T: AsRef<str>>(s: T) {
    /// as_ref : perform the conversion
    assert_eq!("hello", s.as_ref());
}

/// Both `String` and `&str` implement `AsRef<str>` 
/// impl AsRef<str> for str
/// impl AsRef<str> for String
fn main() {
    let s = "hello";
    is_hello(s);

    let s = "hello".to_string();
    is_hello(s);
}