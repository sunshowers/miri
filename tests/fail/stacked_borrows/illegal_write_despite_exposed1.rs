// compile-flags: -Zmiri-permissive-provenance

fn main() {
    unsafe {
        let root = &mut 42;
        let addr = root as *mut i32 as usize;
        let exposed_ptr = addr as *mut i32;
        // From the exposed ptr, we get a new SRO ptr.
        let root2 = &*exposed_ptr;
        // Stack: Unknown(<N), SRO(N), SRO(N+1)
        // And we test that it is read-only by doing a conflicting write.
        *exposed_ptr = 0;
        // Stack: Unknown(<N)
        let _val = *root2; //~ ERROR: borrow stack
    }
}