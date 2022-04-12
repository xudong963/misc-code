use std::marker::{PhantomData, PhantomPinned};
use std::pin::Pin;

struct SelfRef<'a> {
    a: String,
    b: &'a String,
}

fn self_ref() {
    let _a = String::from("hello");
    // let s = SelfRef{a, b: &a};
    /*
     7 |     let a = String::from("hello");
       |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
     8 |     let s = SelfRef{a, b: &a};
       |                     -     ^^ value borrowed here after move
       |                     |
       |                     value moved here
    */
}

struct SelfRefWithPtr {
    a: String,
    /// b point to a and b save a's address in b's stack
    b: *const String,
}

impl SelfRefWithPtr {
    fn new(txt: &str) -> Self {
        SelfRefWithPtr {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.b = &self.a;
    }

    fn a(&self) -> &String {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

fn self_ref_with_ptr() {
    let mut test1 = SelfRefWithPtr::new("test1");
    test1.init();
    let mut test2 = SelfRefWithPtr::new("test2");
    test2.init();
    println!("a: {}, b: {}", test1.a(), test1.b());
    /// swap()函数交换两个Test结构体之后，字段a,b分别移动到对方的内存区域上，但是a和b本身的内容没有变。也就是指针b依然指向的是原来的地址, test1.a
    std::mem::swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();
    /// a: test1, b: I've totally changed now!
    println!("a: {}, b: {}", test2.a(), test2.b());
}

/// Rust的Generator和async/await这一套都是基于自引用结构体实现的
/// `std::mem::swap(&mut test1, &mut test2);` 导致自引用结构体失效，引发了内存安全问题
/// 根源就是在 safe rust 下暴露了: &mut
/// https://folyd.com/blog/rust-pin-unpin/

/// 修复 `self_ref_with_ptr`
struct SelfRefWithHeapPin {
    a: String,
    /// b point to a and b save a's address in b's stack
    b: *const String,
    /// 使用PhantomPinned使 `SelfRefWithStackPin 不会实现 Unpin
    _marker: PhantomPinned,
}

impl SelfRefWithHeapPin {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let self_ref_with_heap_pin = SelfRefWithHeapPin {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(self_ref_with_heap_pin);
        let pointer_to_a: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = pointer_to_a };
        boxed
    }

    fn a(self: Pin<&Self>) -> &String {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

fn self_ref_with_heap_pin() {
    let mut test1 = SelfRefWithHeapPin::new("test1");
    let mut test2 = SelfRefWithHeapPin::new("test2");
    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    // std::mem::swap(&mut *test1, &mut *test2);
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}

/// Pin目前最重要的一个应用：Future. Future 内部有自引用的问题
/// 因为async/await就是通过Generator实现的，Generator是通过匿名结构体实现的。
/// 如果async函数中存在跨await的引用，会导致底层Generator存在跨yield的引用，那根据Generator生成的匿名结构体就会是一个自引用结构体！
/// 然后这个自引用结构体会impl Future，异步的Runtime在调用Future::poll()函数查询状态的时候，需要一个可变借用（即&mut Self）。
/// 如果这个&mut Self不包裹在Pin里面的话，开发者自己impl Future就会利用std::mem::swap()之类的函数move掉&mut Self！
/// 所以这就是Future的poll()必须要使用Pin<&mut Self>的原因。

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
