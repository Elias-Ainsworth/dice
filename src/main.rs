use clap::Parser;
use dice::{completions, roll_dice, web::App, DiceArgs, DiceSubcommand};
use dioxus::{desktop::Config, prelude::LaunchBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DiceArgs::parse();
    if let Some(webapp) = args.launch_webapp {
        match webapp {
            true => {
                LaunchBuilder::desktop()
                    .with_cfg(
                        Config::new()
                            .with_background_color((30, 30, 46, 255))
                            .with_menu(None),
                    )
                    .launch(App);
            }
            false => {
                match args.command.as_ref() {
                    Some(command) => match command {
                        DiceSubcommand::Generate(args) => completions(args),

                        DiceSubcommand::Faces(args) => println!("{}", roll_dice(args)?),
                    },
                    None => roll_dice(&dice::FacesArgs {
                        faces: Some(6),
                        disable_ascii: Some(false),
                    }),
                }?;
            }
        }
    }
    Ok(())
}
