use clap::Parser;
use dice::{completions, roll_dice, DiceArgs, DiceSubcommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DiceArgs::parse();
    match args.command.as_ref() {
        Some(command) => match command {
            DiceSubcommand::Generate(args) => completions(args),
            DiceSubcommand::Faces(args) => roll_dice(args),
        },
        None => roll_dice(&dice::FacesArgs {
            faces: Some(6),
            disable_ascii: Some(false),
        }),
    }?;
    Ok(())
}
