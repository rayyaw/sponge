# Sponge - the package manager manager

Do you have multiple package managers installed on your system and hate keeping track of them individually? If so, Sponge may be the right tool for you!

- Integrates natively with existing package managers
- Plug-and-play approach
- Manage all packages with a single CLI

Sponge is maintained by [rayyaw](https://github.com/rayyaw).

## For users

## For developers

If you are looking to create a tool that allows Sponge to communicate with your package manager of choice, you will need to create a **Sponge plugin**.

### Installing a Sponge plugin

To install a Sponge plugin, you must compile it into a dynamically linked library (either .dll or .so, depending on the platform of choice). To install, you should then place it into the system's `${SPONGE_CONFIG_PATH}/sources` folder. You do not need to include a C header.


### Our API

You can find our API in the `src/api.rs` file. You must implement this API.

### Specifically required methods

1. `plugin_name: extern "C" fn () -> *const libc::c_char`

This should return the name of the package manager as a C-string, like `apt` or `flatpak`.

2. All methods ending in `_version`

This should return an integer corresponding to the version of the API that you have implemented for that method. If you are creating a new project, you should always use the latest version of the API.

### `upgrade` API

todo

### `install` API

todo: explain all or nothing and this will only work if you're installing all packages from the same package manager