// drive2.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


struct Foo {
    a: u128,
    b: Option<String>,
}

fn raw_pointer_to_box(address: usize) -> Box<Foo> {
    // Convert the raw pointer back to a Box<Foo>
    let mut foo_boxed = unsafe { Box::from_raw(address as *mut Foo) };

    // Modify the b field of the Foo structure
    foo_boxed.b = Some("hello".to_owned());

    foo_boxed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let nanos = Instant::now().duration_since(Instant::now().checked_sub(std::time::Duration::new(0, 1)).unwrap()).as_nanos();

        let data = Box::new(Foo{
            a: nanos,
            b: None
        });

        let ptr_1 = &data.a as *const u128 as usize;
        let ret = raw_pointer_to_box(Box::into_raw(data) as usize);

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));

    }
}
