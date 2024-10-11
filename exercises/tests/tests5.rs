// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // Safety: The caller of this function must ensure that `address` is valid and
    // points to a writable `u32` location.
    unsafe {
        // Convert the usize back into a mutable raw pointer to a u32.
        let ptr = address as *mut u32;
        
        // Dereference the pointer and modify the value.
        // For example, let's set the value to 42.
        if !ptr.is_null() {
            *ptr = 0xAABBCCDD;
        } else {
            // If the pointer is null, we should handle it appropriately.
            // In this case, it's an unsafe operation, so handling null pointers is important.
            panic!("Null pointer dereference");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
