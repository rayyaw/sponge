use std::ffi::CStr;

use dlopen2::wrapper::Container;
use indicatif::{ProgressBar, ProgressStyle};

use crate::api::PackageManagerApi;

pub fn update(args: Vec<String>, plugins: Vec<Container<PackageManagerApi>>) {
    if args.len() == 0 {
        update_all(plugins);
    }

    // TODO: Else
}

fn update_all(plugins: Vec<Container<PackageManagerApi>>) {
    // Setup progress bar
    let progress_style = ProgressStyle::default_bar()
        .template("[{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Unable to create progress bar!");

    // Set up a progress bar
    let progress_bar = ProgressBar::new(plugins.len().try_into().unwrap());
    progress_bar.set_style(progress_style);

    for plugin in plugins {
        // Get the plugin name
        let plugin_name_raw = plugin.plugin_name();
        let plugin_name = unsafe { CStr::from_ptr(plugin_name_raw) }.to_string_lossy();

        // Get the API version, then call the corresponding function
        let api_version = plugin.upgrade_all_api_version();

        // Call the correct API version
        match api_version {
            1 => {
                plugin.upgrade_all_api_v1().expect(&format!(
                    "API Error: Could not find the upgrade all API v1 of {}!", plugin_name
                ))
            }

            _ => { panic!(
                "API Error: The upgrade all API of {} uses undefined version {}!",
                plugin_name,
                api_version,
            )}
        };

        // Increment the progress bar
        progress_bar.inc(1);
    }
}
