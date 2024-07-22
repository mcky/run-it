use strum::EnumIter;

#[derive(clap::ValueEnum, Eq, PartialEq, Hash, Clone, Debug, EnumIter)]
pub enum Tools {
    Npm,
    Pnpm,
    Yarn,
    Make,
    Mise,
    Just,
    TurboRepo,
    Mix,
}

pub fn match_file_to_tool(file_name: &str) -> Option<Tools> {
    match file_name {
        "Makefile" => Some(Tools::Make),
        "pnpm-lock.yaml" => Some(Tools::Pnpm),
        "yarn.lock" => Some(Tools::Yarn),
        "mise.toml" => Some(Tools::Mise),
        "package-lock.json" => Some(Tools::Npm),
        "package.json" => Some(Tools::Npm),
        "justfile" => Some(Tools::Just),
        "turbo.json" => Some(Tools::TurboRepo),
        "mix.exs" => Some(Tools::Mix),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn all_tools_are_matched() {
        let all_tools = Tools::iter().collect::<Vec<_>>();

        for tool in &all_tools {
            let file_name = match tool {
                Tools::Make => "Makefile",
                Tools::Pnpm => "pnpm-lock.yaml",
                Tools::Yarn => "yarn.lock",
                Tools::Npm => "package-lock.json",
                Tools::Mise => "mise.toml",
                Tools::Just => "justfile",
                Tools::TurboRepo => "turbo.json",
                Tools::Mix => "mix.exs",
            };

            let matched = match_file_to_tool(file_name);
            assert_eq!(
                matched,
                Some(tool.clone()),
                "expected {file_name} to resolve to {tool:?}",
            );
        }
    }
}
