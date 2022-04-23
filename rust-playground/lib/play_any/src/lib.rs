use std::any::Any;

trait A {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct B {
    b: i32,
}

impl B {
    fn set_b(&mut self, b: i32) {
        self.b = b;
    }
}

impl A for B {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let mut a: Box<dyn A> = Box::new(B { b: 1 });

    let child = match a.as_any_mut().downcast_mut::<B>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };

    child.set_b(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
