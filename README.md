# AI Integrations
This software permit you to integrate AI in your mondain pc usage letting you build your own AI shortcuts and commands with a simple to use node system.

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

# Development
## Requirements
- Nix
- direnv (optional but recommended)
## Run
```bash
# Inside the nix flakes shell (automatically loaded if using direnv)
bun run tauri dev
```
