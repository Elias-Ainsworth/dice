use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Subcommand, ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum ShellCompletion {
    Bash,
    Zsh,
    Fish,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
    #[arg(value_enum, help = "Type of Shell completion to generate")]
    pub shell: ShellCompletion,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct FacesArgs {
    #[arg(name = "NUMBER", help = "Number of dice faces to roll.")]
    pub faces: Option<i32>,

    #[arg(long, short = 'a', default_value = "false")]
    pub disable_ascii: Option<bool>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum DiceSubcommand {
    #[command(name = "generate", about = "Generate shell completions", hide = true)]
    Generate(GenerateArgs),
    #[command(name = "faces", about = "Number of die faces.")]
    Faces(FacesArgs),
}

#[derive(Parser, Debug)]
#[command(name = "dice-rs", about = "A simple dice app written in rust")]
pub struct DiceArgs {
    #[command(subcommand)]
    pub command: Option<DiceSubcommand>,

    #[arg(long, short = 'w', default_value = "false")]
    pub launch_webapp: Option<bool>,
}
