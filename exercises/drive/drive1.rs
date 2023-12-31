// drive1.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn modify_by_address(address: usize) {
    // Convert the `usize` to a mutable raw pointer.
    let ptr = address as *mut u32;

    // Use an `unsafe` block to modify the data at that address.
    unsafe {
        *ptr = 0xAABBCCDD;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t:u32 = 0x12345678;
        modify_by_address(&mut t as *mut u32 as usize);
        assert!(t == 0xAABBCCDD);
    }
}
