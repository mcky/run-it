use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

#[derive(clap::ValueEnum, Clone, Debug)]
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
        task: Option<String>,

        #[arg(long, short)]
        dir: PathBuf,

        #[arg(short, long)]
        tool: Option<Tools>,
    },
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
        .ok()
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
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { task, dir, .. }) => {
            let tools = scan_for_tools(dir);
            println!(
                "'run-it run' was used, task is: {:?}, tools are {:?}",
                task, tools
            )
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}
