use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{env, fs, path::PathBuf};

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
        Some(Commands::Run {
            task,
            dir,
            var_args,
            ..
        }) => run_task_cmd(task, dir, var_args),
        None => {
            todo!("Needs a sub-command");
        }
    }

    Ok(())
}

fn run_task_cmd(task: &String, dir: &PathBuf, var_args: &Vec<String>) {
    let tools = scan_for_tools(dir);
    println!(
        "'run-it run' was used, task is: {task:?}, tools are {tools:?}, var_args are {var_args:?}, in dir {dir:?}",
    );

    let var_args_str = var_args.join(" ");

    match &tools[..] {
        [tool] => {
            run_task_for_tool(tool.clone(), task.clone(), var_args_str);
        }
        [] => {
            todo!("No tool provided")
        }
        _ => todo!("multi tool handling"),
    }

    ()
}

fn build_run_cmd(tool: Tools, task: String, var_args: String) -> String {
    return match tool {
        Tools::Make => format!("make {task} {var_args}"),
        Tools::Npm => format!("npm run {task} {var_args}"),
        Tools::Pnpm => format!("pnpm run {task} {var_args}"),
        Tools::Yarn => format!("yarn run {task} {var_args}"),
        Tools::Mise => format!("mise run {task} {var_args}"),
    };
}

fn run_task_for_tool(tool: Tools, task: String, var_args: String) {
    let cmd = build_run_cmd(tool, task, var_args);

    println!("running: {cmd}");

    let result = std::process::Command::new("sh")
        .arg("-c")
        .current_dir("./examples/make")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    println!("{result:?}");
}
