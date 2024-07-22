#[derive(clap::ValueEnum, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Tools {
    Npm,
    Pnpm,
    Yarn,
    Make,
    Mise,
}

pub fn match_file_to_tool(file_name: &str) -> Option<Tools> {
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
