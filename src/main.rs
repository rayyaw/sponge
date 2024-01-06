use libc::{c_void, dlopen, dlsym, dlclose};
use std::env;
use std::ffi::CString;
use std::fs;
use std::path::Path;

// sponge install x y z (including from file, like deb or flatpak)
// sponge update
// sponge remove x y z
// future work: support only updating specified packages
// future work: support autoremove
// future work: support adding/removing PPAs or equivalent sources
// future work: support search

struct PackageManager {
    bridge_library: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
        return ();
    }

    let config_path: String = "./sponge".to_string();

    // Read $CONFIG_PATH/sources to find all installed package managers
    // Each package manager will have a .so file with the required functions
    if let Ok(sources) = fs::read_dir(config_path.clone() + "/sources") {
        for entry in sources {
            if let Ok(entry) = entry {
                let dll_path = CString::new(entry.path().to_str()
                    .expect("Invalid characters in your dynamic libraries!"))
                    .expect("Invalid characters in your dynamic libraries!");

                let library = unsafe { 
                    dlopen(dll_path.as_ptr(), libc::RTLD_NOW);
                };

                // FIXME
                if library.is_null() {
                    eprintln!("Failed to load the shared library: {:?}", dll_path);
                    return;
                }

                let file_name = entry.file_name();
                println!("{}", file_name.to_string_lossy());
            }
        }
    } else {
        eprintln!("sponge: Could not find sources at {}.
        Please ensure you have a valid sponge config.", config_path.clone());
    }

}

fn print_usage() {
    println!("sponge {} - the package manager manager", env!("CARGO_PKG_VERSION"));

    println!(
"Implemented commands:
    install - install packages
    remove - remove packages
    update - update packages")
}