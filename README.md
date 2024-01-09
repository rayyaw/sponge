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

You can find our API in the `src/api.rs` file. You must implement this API 