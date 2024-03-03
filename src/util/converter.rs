pub fn vec_to_ptr(vec: &Vec<String>) -> (*const *const libc::c_char, usize) {
    // Convert Vec<String> to Vec<*const c_char>
    let ptrs: Vec<*const c_char> = vec.into_iter()
        .map(|s| CString::new(s).unwrap().into_raw() as *const c_char)
        .collect();

    let size = ptrs.len();

    // Obtain a raw pointer to the array
    let raw_ptr = ptrs.as_ptr();

    // Return the raw pointer and the size of the array
    (raw_ptr, size)
}