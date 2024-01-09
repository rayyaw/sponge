use dlopen2::wrapper::Container;
use std::env;
use std::fs;
use std::process;

mod api;
use api::PackageManagerApi;

mod commands {
    pub mod update;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
        return ();
    }

    let config_path: String = "./sponge".to_string();

    // Read $CONFIG_PATH/sources to find all installed package managers
    // Each package manager will have a .so/.dll file with the required functions
    let sponge_plugins: Vec<Container<PackageManagerApi>> = 
        match fs::read_dir(config_path.clone() + "/sources") {

        Ok(sources) => {
            sources
                .filter_map(|entry| entry.ok()) // Filter out errors
                .filter_map(|entry| {
                    let library_path = entry.path();
                    let library_name = library_path.to_str()
                        .expect("Invalid characters in dynamic libraries!");
                    
                    unsafe {
                        Container::load(library_name)
                            .map_err(|_| eprintln!("Could not load dynamic library: {}", library_name.to_owned()))
                            .ok()
                    }
                })
                .collect()
        }
        
        Err(_) => {
            eprintln!("sponge: Could not find sources at {}. Please ensure you have a valid sponge config.", config_path.clone());
            process::exit(126);
        }
    };

    // TODO - command processing
    // sponge install x y z (including from file, like deb or flatpak)
    // sponge update
    // sponge remove x y z
    // future work: support only updating specified packages
    // future work: support autoremove
    // future work: support adding/removing PPAs or equivalent sources
    // future work: support search
    match args[1].as_str() {
        "update" => { commands::update::update(args[2..].to_vec(), sponge_plugins) }
        _ => {
            eprintln!("Unknown command: {}. Type 'sponge' to get a list of available commands.", args[1]);
            process::exit(127);
        }
    }
    
}

fn print_usage() {
    println!("sponge {} - the package manager manager", env!("CARGO_PKG_VERSION"));

    println!(
"Implemented commands:
    update - update packages")
}