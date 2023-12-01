# AI Integrations
This software permit you to integrate AI in your mondain pc usage letting you build your own AI shortcuts and commands with a simple-to-use node system.

# Compatibility
- Linux
  - X11 (non-tiling): full support
  - X11 (tiling): may not work, may be fixable using custom rules
  - Wayland
    - Hyprland: full support
    - Other (non-tiling): support using xWayland
- Windows
- MacOS

# Plugins - WIP
Plugins can be written using WASM

# Plugin development
TODO

# Branches
- [master](https://github.com/Dreaming-Codes/ai-integrations/tree/master): where new features are merged
- [temp-openai](https://github.com/Dreaming-Codes/ai-integrations/tree/temp-openai): this is a temporary implementation of openai API (not intended to be merged), it will be removed as soon as the plugin system is ready and replaced with an openai plugin
- [other branches](https://github.com/Dreaming-Codes/ai-integrations/branches): refer to [CONTRIBUTING.md](./.github/CONTRIBUTING.md)

# Development
## Requirements
- NixCONTRIBUTING.mdt
- direnv (optional but recommended)
## Run
Remember to also run your IDE inside the nix flakes shell, otherwise it won't be able to find the dependencies.
If using IntelliJ or any other JetBrains IDE remember to rerun `direnv allow` in the IDE terminal every time you open the project since there's a bug that prevents it from loading the nix flakes shell automatically when the IDE is started inside a direnv shell.
```bash
# Inside the nix flakes shell (automatically loaded if using direnv)
bun run tauri dev
```

# Contributing
See [CONTRIBUTING.md](./.github/CONTRIBUTING.md) for more information.
