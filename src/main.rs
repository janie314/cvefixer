use clap::{Parser, Subcommand};
use config::{read_config, Config};
use log::{info, LevelFilter};
use std::env;
use task::{bun, deno, flatpak, hooks, os, ruby, rust, uv};

mod config;
mod errors;
mod helpers;
mod task;

/// fix all vulnerabilities that are fixed
#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all update tasks
    All {},
    /// Update Bun
    Bun {},
    /// Update Deno
    Deno {},
    /// Update Flatpak packages
    Flatpak {},
    /// Run configured hook scripts
    Hooks {},
    /// Update OS packages (supports RHEL-like, Gentoo, Ubuntu)
    OS {},
    /// Update Rust and Cargo packages
    Rust {},
    /// Update Ruby and Gems
    Ruby {},
    /// Test command for development
    Test {},
    /// Update uv
    Uv {},
}

fn run_hooks(config: Option<Config>) {
    match config {
        Some(config) => {
            hooks::update(config).expect("there was an issue");
        }
        None => {
            info!("no hooks configured");
        }
    }
}

fn main() {
    match env::var("RUST_LOG") {
        Ok(_) => pretty_env_logger::init(),
        Err(_) => pretty_env_logger::formatted_builder()
            .filter(None, LevelFilter::Info)
            .init(),
    }
    // parse CLI args
    let args = Args::parse();
    let config = read_config().expect("there was an issue reading the configuration file");
    match &args.command {
        Some(cmd) => match cmd {
            Commands::All {} => {
                rust::update().expect("there was an issue");
                os::update().expect("there was an issue");
                deno::update().expect("there was an issue");
                bun::update().expect("there was an issue");
                ruby::update().expect("there was an issue");
                flatpak::update().expect("there was an issue");
                run_hooks(config);
            }
            Commands::Bun {} => {
                bun::update().expect("there was an issue");
            }
            Commands::Deno {} => {
                deno::update().expect("there was an issue");
            }
            Commands::Flatpak {} => {
                flatpak::update().expect("there was an issue");
            }
            Commands::Hooks {} => run_hooks(config),
            Commands::OS {} => {
                os::update().expect("there was an issue");
            }
            Commands::Ruby {} => {
                ruby::update().expect("there was an issue");
            }
            Commands::Rust {} => {
                rust::update().expect("there was an issue");
            }
            Commands::Test {} => {
                info!("testing...");
            }
            Commands::Uv {} => {
                uv::update().expect("there was an issue");
            }
        },
        None => (),
    }
}
