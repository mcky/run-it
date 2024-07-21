use anyhow::Result;
use clap::{Parser, Subcommand};

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

        #[arg(short, long)]
        tool: Option<Tools>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { task, .. }) => {
            println!("'run-it run' was used, task is: {:?}", task)
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}
