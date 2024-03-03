use std::ffi::{CStr, CString};

use dlopen2::wrapper::Container;
use libc::c_char;

use crate::api::PackageManagerApi;

pub fn bulk_install(args: Vec<String>, plugins: Vec<Container<PackageManagerApi>>) {
    let (package_list, num_packages) = vec_to_ptr(&args);

    for plugin in plugins {
        let plugin_name_raw = plugin.plugin_name();
        let plugin_name = unsafe { CStr::from_ptr(plugin_name_raw) }.to_string_lossy();
        let api_version = plugin.bulk_install_api_version();

        let package_c_str = unsafe { *package_list.offset(0 as isize) };
        print_i8_ptr(package_c_str); // FIXME - remove this line
        let package = unsafe { CStr::from_ptr(package_c_str).to_string_lossy() };
        let package_str = package.as_ref();
        eprintln!("package = {package_str}");

        let success = match api_version {
            1 => {
                plugin.bulk_install_api_v1(package_list, num_packages)
                    .expect(&format!(
                        "API Error: Could not find the bulk install API v1 of {}!", plugin_name
                    ))
            }

            _ => { panic!(
                "API Error: The bulk install API of {} uses undefined version {}!",
                plugin_name,
                api_version,
            )}
        };

        // FIXME - break on success
    }

    // FIXME - error handling
}

// FIXME - move these back to util
// FIXME - this is deallocated at the end of this fn
pub fn vec_to_ptr(vec: &Vec<String>) -> (*const *const c_char, usize) {
    // Convert Vec<String> to Vec<*const c_char>
    let ptrs: Vec<*const c_char> = vec.into_iter()
        .map(|s| CString::new(s.clone())
            .unwrap()
            .into_raw()
            as *const c_char
        )
        .collect();

    let size = ptrs.len();

    // Obtain a raw pointer to the array
    let raw_ptr = ptrs.as_ptr();

    // Return the raw pointer and the size of the array
    (raw_ptr, size)
}

fn print_i8_ptr(ptr: *const i8) {
    unsafe {
        let mut current_ptr = ptr;
        loop {
            let current_char = *current_ptr;
            if current_char == 0 {
                break; // Null terminator found, end loop
            }
            eprint!("{}", current_char as u8 as char); // Print character
            current_ptr = current_ptr.add(1); // Move to the next character
        }
    }
}