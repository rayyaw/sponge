use dlopen2::wrapper::Container;

use crate::api::PackageManagerApi;

pub fn update(args: Vec<String>, plugins: Vec<Container<PackageManagerApi>>) {
    if args.len() == 0 {
        update_all(plugins);
    }

    // TODO: Else
}

fn update_all(plugins: Vec<Container<PackageManagerApi>>) {
    // TODO - Implement
    for plugin in plugins {
        println!("{:?}", plugin.plugin_name());
    }
}