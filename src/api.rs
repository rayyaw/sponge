use dlopen2::wrapper::WrapperApi;

// The API for the package manager. This must be implemented within your .so/.dll file.
#[derive(WrapperApi)]
pub struct PackageManagerApi {
    // Plugin Name
    // Should be the name of the package manager in use, like `apt`
    plugin_name: extern "C" fn () -> *const libc::c_char,

    // Upgrade API
    upgrade_all_api_version: extern "C" fn () -> u32,
    upgrade_all_api_v1: Option<extern "C" fn () -> ()>,

    // Installation API
    bulk_install_api_version: extern "C" fn () -> u32,
    bulk_install_api_v1: Option<
        extern "C" fn (packages: *const *const libc::c_char, num_packages: usize) -> bool
    >
}