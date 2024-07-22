pub mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{collections::HashSet, env, fs, path::PathBuf};

#[derive(clap::ValueEnum, Eq, PartialEq, Hash, Clone, Debug)]
enum Tools {
    Npm,
    Pnpm,
    Yarn,
    Make,
    Mise,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// run the script named <task>
    Run {
        /// The task to run (e.g. dev, build, run)    
        task: String,

        /// Which folder to scan, defaults to CWD
        #[arg(
            long, short,
            default_value_os_t = get_default_dir()
        )]
        #[arg(long, short)]
        dir: PathBuf,

        /// Explicitly override the tool instead of inferring it from $dir
        #[arg(short, long)]
        tool: Option<Tools>,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
        var_args: Vec<String>,
    },
}

fn get_default_dir() -> PathBuf {
    return env::current_dir().expect("could not infer CWD");
}

fn match_file_to_tool(file_name: &str) -> Option<Tools> {
    match file_name {
        "Makefile" => Some(Tools::Make),
        "pnpm-lock.yaml" => Some(Tools::Pnpm),
        "yarn.lock" => Some(Tools::Yarn),
        "mise.toml" => Some(Tools::Mise),
        "package-lock.json" => Some(Tools::Npm),
        "package.json" => Some(Tools::Npm),
        _ => None,
    }
}

fn scan_for_tools(dir: &PathBuf) -> Vec<Tools> {
    fs::read_dir(dir)
        .into_iter()
        .flat_map(|entries| entries.filter_map(|entry| entry.ok()))
        .filter_map(|entry| {
            entry.path().file_name().and_then(|file_name| {
                file_name
                    .to_str()
                    .map(|file_name_str| match_file_to_tool(&file_name_str))
            })
        })
        .flatten()
        .into_iter()
        .collect::<HashSet<Tools>>()
        .into_iter()
        .collect::<Vec<Tools>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run {
            task,
            dir,
            var_args,
            ..
        }) => commands::run::exec(task, dir, var_args),
        None => {
            todo!("Needs a sub-command");
        }
    }

    Ok(())
}
