use crate::{get_default_dir, scan_for_tools, tools::Tools};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Run {
    /// The task to run (e.g. dev, build, run)    
    pub(crate) task: String,

    /// Which folder to scan, defaults to CWD
    #[arg(
        long, short,
        default_value_os_t = get_default_dir()
    )]
    #[arg(long, short)]
    pub(crate) dir: PathBuf,

    /// Explicitly override the tool instead of inferring it from $dir
    #[arg(short, long)]
    pub(crate) tool: Option<Tools>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub(crate) var_args: Vec<String>,
}

pub fn exec(task: &String, dir: &PathBuf, var_args: &Vec<String>) {
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
        Tools::Mise => format!("mise run {task} {var_args}")
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
