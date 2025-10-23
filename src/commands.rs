use crate::config::{CONFIG_FILE, DevShellConfig};
use crate::flake_editor;
use crate::templates::{FLAKE_FILE, generate_flake_nix};
use crate::{AddArgs, InitArgs, parse_flake_input, parse_input_to_pkg_string};
use std::collections::BTreeMap;
use std::fs;
use std::io::Error;

/**
 * @brief Handles the `nix-shell-gen init` command.
 *
 * Initializes a new development shell by generating `flake.nix` and `devshell.toml`
 * files based on the provided arguments.
 *
 * @param args Arguments for initialization.
 * @return Result<(), Error> Returns Ok on success, or an Error if initialization fails.
 */
pub fn handle_init(args: &InitArgs) -> Result<(), Error> {
    if !args.force && (fs::metadata(FLAKE_FILE).is_ok() || fs::metadata(CONFIG_FILE).is_ok()) {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "{} or {} already exists. Use --force to overwrite.",
                FLAKE_FILE, CONFIG_FILE
            ),
        ));
    }

    // Prepare flake.nix inputs
    let mut flake_inputs = BTreeMap::new();
    for url in &args.inputs {
        let (key, url_str) = parse_flake_input(url);
        flake_inputs.insert(key, url_str);
    }

    // Write flake.nix
    let flake_content = generate_flake_nix(&flake_inputs);
    fs::write(FLAKE_FILE, flake_content)?;
    println!("Created {}.", FLAKE_FILE);

    // Prepare devshell.toml config
    let mut config = DevShellConfig::default();

    // Add language-specific packages
    if let Some(lang) = &args.lang {
        match lang.to_lowercase().as_str() {
            "cpp" | "c++" => {
                config.packages.insert("clang".to_string());
                config.packages.insert("cmake".to_string());
                config.packages.insert("gdb".to_string());
            }
            "rust" => {
                config.packages.insert("rustc".to_string());
                config.packages.insert("cargo".to_string());
                config.packages.insert("rust-analyzer".to_string());
            }
            "python" => {
                config.packages.insert("python3".to_string());
            }
            _ => println!("Warning: Unknown language template '{}'", lang),
        }
    }

    // Add user-specified packages
    config.packages.extend(args.packages.iter().cloned());

    // Add packages from flake inputs
    for url in &args.inputs {
        config.packages.insert(parse_input_to_pkg_string(url));
    }

    // Add shell hook
    if let Some(hook) = &args.shell_hook {
        config.append_hook(hook);
    }

    // Set purity
    if args.isolated {
        config.pure = Some(true);
    }

    // Write devshell.toml
    config.save()?;
    println!("Created {}.", CONFIG_FILE);

    Ok(())
}

/**
 * @brief Handles the `nix-shell-gen add` command.
 *
 * Adds packages, flake inputs, or shell hooks to an existing development shell configuration.
 *
 * @param args Arguments for adding packages, inputs, or hooks.
 * @return Result<(), Error> Returns Ok on success, or an Error if the operation fails.
 */
pub fn handle_add(args: &AddArgs) -> Result<(), Error> {
    let mut config = DevShellConfig::load()?;

    // Handle Flake Inputs (-P)
    if !args.inputs.is_empty() {
        println!("Adding new flake inputs to {}...", FLAKE_FILE);
        for url in &args.inputs {
            let (key, url_str) = parse_flake_input(url);

            // Attempt to add the flake input to flake.nix
            match flake_editor::add_flake_input(&key, &url_str) {
                Ok(_) => {
                    println!("Successfully added input '{}' to {}.", key, FLAKE_FILE);
                    // Add the package from the input to the config
                    config.packages.insert(parse_input_to_pkg_string(url));
                }
                Err(e) => {
                    eprintln!("Failed to add input '{}' to {}: {}", key, FLAKE_FILE, e);
                    eprintln!(
                        "Please add it manually: inputs.{}.url = \"{}\";",
                        key, url_str
                    );
                }
            }
        }
    }

    // Add packages (-p)
    if !args.packages.is_empty() {
        let count_before = config.packages.len();
        config.packages.extend(args.packages.iter().cloned());
        let added_count = config.packages.len() - count_before;
        println!("Added {} new packages to {}.", added_count, CONFIG_FILE);
    }

    // Add shell hook (-s)
    if let Some(hook) = &args.shell_hook {
        config.append_hook(hook);
        println!("Appended shell hook to {}.", CONFIG_FILE);
    }

    config.save()?;
    println!("Updated {}.", CONFIG_FILE);

    Ok(())
}
