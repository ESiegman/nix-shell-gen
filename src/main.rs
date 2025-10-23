use clap::{Parser, Subcommand};

mod commands;
mod config;
mod flake_editor;
mod templates;

/**
 * @brief A CLI to declaratively generate and manage Nix flake development shells.
 */
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/**
 * @enum Commands
 * @brief Supported subcommands for the CLI.
 */
#[derive(Subcommand, Debug)]
enum Commands {
    /**
     * @brief Initialize a new dev shell flake.
     * @details Creates flake.nix and devshell.toml.
     */
    Init(InitArgs),

    /**
     * @brief Add packages or hooks to an existing devshell.toml.
     */
    Add(AddArgs),
}

/**
 * @struct InitArgs
 * @brief Arguments for the `init` subcommand.
 */
#[derive(Parser, Debug)]
struct InitArgs {
    /**
     * @brief The primary language template (e.g., "cpp", "rust", "python").
     */
    #[arg(short = 'l', long)]
    lang: Option<String>,

    /**
     * @brief Extra Nixpkgs packages to add (space-separated).
     */
    #[arg(short = 'p', long, value_delimiter = ' ', num_args = 0..)]
    packages: Vec<String>,

    /**
     * @brief Extra flake inputs to add (space-separated URLs).
     * @details Example: "github:nix-community/crane" "github:ocornut/imgui"
     */
    #[arg(short = 'P', long, value_delimiter = ' ', num_args = 0..)]
    inputs: Vec<String>,

    /**
     * @brief A shell hook command to run.
     */
    #[arg(short = 's', long)]
    shell_hook: Option<String>,

    /**
     * @brief Create an isolated, pure shell (default is an impure shell).
     */
    #[arg(long)]
    isolated: bool,

    /**
     * @brief Overwrite existing flake.nix and devshell.toml.
     */
    #[arg(long)]
    force: bool,
}

/**
 * @struct AddArgs
 * @brief Arguments for the `add` subcommand.
 */
#[derive(Parser, Debug)]
struct AddArgs {
    /**
     * @brief Nixpkgs packages to add (space-separated).
     */
    #[arg(short = 'p', long, value_delimiter = ' ', num_args = 0..)]
    packages: Vec<String>,

    /**
     * @brief Flake inputs to add (space-separated URLs).
     * @details This will automatically edit your flake.nix.
     */
    #[arg(short = 'P', long, value_delimiter = ' ', num_args = 0..)]
    inputs: Vec<String>,

    /**
     * @brief A shell hook command to append.
     */
    #[arg(short = 's', long)]
    shell_hook: Option<String>,
}

/**
 * @brief Entry point for the CLI application.
 */
fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init(args) => commands::handle_init(args),
        Commands::Add(args) => commands::handle_add(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/**
 * @brief Parses a flake URL into a (key, url) tuple.
 * @details
 * Example: "github:owner/repo" -> ("repo", "github:owner/repo")
 * Example: "github:owner/repo~branch" -> ("repo", "github:owner/repo~branch")
 * @param url The flake URL to parse.
 * @return A tuple containing the key and the original URL.
 */
fn parse_flake_input(url: &str) -> (String, String) {
    let key = url
        .split('/')
        .last()
        .unwrap_or(url)
        .split('~')
        .next()
        .unwrap_or(url)
        .to_string();
    (key, url.to_string())
}

/**
 * @brief Parses a flake URL into a package string.
 * @details
 * Example: "github:owner/repo" -> "repo.packages.${system}.default"
 * @param url The flake URL to parse.
 * @return The package string following the standard convention for flake packages.
 */
fn parse_input_to_pkg_string(url: &str) -> String {
    let (key, _) = parse_flake_input(url);
    format!("{}.packages.${{system}}.default", key)
}
