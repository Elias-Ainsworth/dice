use crate::cli::{DiceArgs, GenerateArgs, ShellCompletion};
use clap::CommandFactory;
use clap_complete::{generate, Shell};

pub fn completions(args: &GenerateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = DiceArgs::command();
    match &args.shell {
        ShellCompletion::Bash => {
            generate(Shell::Bash, &mut cmd, "mkdevenv", &mut std::io::stdout());
        }
        ShellCompletion::Zsh => {
            generate(Shell::Zsh, &mut cmd, "mkdevenv", &mut std::io::stdout());
        }
        ShellCompletion::Fish => {
            generate(Shell::Fish, &mut cmd, "mkdevenv", &mut std::io::stdout());
        }
    }
    Ok(())
}
