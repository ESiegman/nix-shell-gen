# nix-shell-gen

**nix-shell-gen** is a CLI tool to declaratively generate and manage [Nix flake](https://nixos.wiki/wiki/Flakes) development shells. It helps you quickly scaffold and evolve reproducible development environments using `flake.nix` and `devshell.toml`, with support for language templates, custom packages, shell hooks, and additional flake inputs.

---

## Features

- **Easy Initialization:** Scaffold a new flake-based dev shell with language templates (Rust, C++, Python, etc).
- **Declarative Management:** Add packages, flake inputs, and shell hooks to your devshell configuration.
- **Safe Editing:** Automatically edits `flake.nix` and `devshell.toml` for you.
- **Composable:** Designed to be used standalone or as a flake input in other projects.

---

## Adding nix-shell-gen to Your Flake

To use `nix-shell-gen` as a flake input in your own project and have it available directly in your `$PATH`, you have several options:

---

### **A. Install Globally (Recommended for Most Users)**

```sh
nix profile install github:esiegman/nix-shell-gen
```
After this, you can run `nix-shell-gen` from any shell, anywhere.

---

### **B. Add to a Project's devShell (for Development Environments)**

Add `nix-shell-gen` as an input in your `flake.nix`:

```nix
{
  inputs.nix-shell-gen.url = "github:esiegman/nix-shell-gen";
  # ... other inputs

  outputs = { self, nixpkgs, nix-shell-gen, ... }:
    let
      system = builtins.currentSystem;
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          nix-shell-gen.packages.${system}.default
          # ...other dev tools
        ];
      };
    };
}
```
Now, after running `nix develop`, you can use `nix-shell-gen` directly in your shell.

---

### **C. Add System-wide (NixOS)**

To make `nix-shell-gen` available everywhere on your system, add it to your `environment.systemPackages` in your NixOS configuration:

```nix
{
  environment.systemPackages = [
    inputs.nix-shell-gen.packages.${pkgs.system}.default
  ];
}
```
After a `nixos-rebuild switch`, `nix-shell-gen` will be available globally.

---

## Building & Installing from Scratch

To build the CLI locally:

```sh
# Clone the repo
git clone https://github.com/esiegman/nix-shell-gen.git
cd nix-shell-gen

# Build with Nix (recommended)
nix build

# Or build with Cargo (requires Rust toolchain)
cargo build --release

# Run the CLI
./target/release/nix-shell-gen --help
```

---

## CLI Usage

### Initialize a New Dev Shell

```sh
nix-shell-gen init [OPTIONS]
```

**Options:**
- `-l, --lang <LANG>`: Language template (`cpp`, `rust`, `python`, ...)
- `-p, --packages <PKGS...>`: Extra Nixpkgs packages (space-separated)
- `-P, --inputs <URLS...>`: Extra flake inputs (e.g. `github:nix-community/crane`)
- `-s, --shell-hook <CMD>`: Shell hook command to run
- `--isolated`: Create a pure shell (default: impure)
- `--force`: Overwrite existing `flake.nix` and `devshell.toml`

### Add Packages, Inputs, or Hooks

```sh
nix-shell-gen add [OPTIONS]
```

**Options:**
- `-p, --packages <PKGS...>`: Nixpkgs packages to add
- `-P, --inputs <URLS...>`: Flake inputs to add (edits `flake.nix`)
- `-s, --shell-hook <CMD>`: Append a shell hook command

---

## Example Workflows

### 1. Initialize a Rust Dev Shell

```sh
nix-shell-gen init --lang rust --packages openssl pkg-config
```
Creates a `flake.nix` and `devshell.toml` with Rust tools and extra packages.

### 2. Add a Flake Input and Use Its Package

```sh
nix-shell-gen add --inputs github:nix-community/crane
```
Adds `crane` as a flake input and makes its default package available in your shell.

### 3. Add a Shell Hook

```sh
nix-shell-gen add --shell-hook 'echo "Welcome to your dev shell!"'
```

### 4. Add More Packages

```sh
nix-shell-gen add --packages gdb valgrind
```

---

## Generated Files

- **flake.nix:** Nix flake definition, generated and updated automatically.
- **devshell.toml:** Declarative list of packages, shell hooks, and purity flag.

---

## Advanced: Customizing the Flake

You can manually edit `flake.nix` and `devshell.toml` for advanced use cases. The CLI will attempt to preserve your changes where possible.

---

## Contributing

Contributions, bug reports, and feature requests are welcome! Please open an issue or PR on GitHub.

---

## License

MIT License © 2025 Eren Siegman

---
