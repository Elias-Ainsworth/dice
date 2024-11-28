use rand::Rng;
use serde::Deserialize;
use std::fs;

use crate::FacesArgs;

#[derive(Deserialize)]
struct DiceFaces {
    faces: std::collections::HashMap<String, String>,
}

fn load_dice_faces(file_path: &str) -> Result<DiceFaces, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let dice_faces: DiceFaces = serde_json::from_str(&data)?;
    Ok(dice_faces)
}

pub fn roll_dice(arg: &FacesArgs) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let faces = match arg.faces {
        Some(faces) => faces,
        None => 6,
    };

    let roll = rng.gen_range(1..=faces);
    let roll_string = format!("{}", roll);
    let roll_str: &str = &roll_string;

    match arg.disable_ascii {
        Some(true) => print_figlet(roll_str),
        _ => {
            if faces <= 6 {
                print_ascii(roll_str)
            } else {
                print_figlet(roll_str)
            }
        }
    }
}

fn print_figlet(roll_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let standard_font = figlet_rs::FIGfont::standard()?;
    let figure = standard_font.convert(roll_str);
    // match figure {
    //     Some(fig) => Ok(format!("{}", fig)),
    //     None => Err(Box::new(std::io::Error::new(
    //         std::io::ErrorKind::InvalidData,
    //         "Failed to convert string to figlet",
    //     ))),
    // }
    figure
        .map(|fig| format!("{}", fig))
        .ok_or_else(|| "Failed to convert string into figlet".into())
}

fn print_ascii(roll_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dice_faces = load_dice_faces("faces.json")?;
    // if let Some(face) = dice_faces.faces.get(roll_str) {
    //     Ok(format!("{}", face))
    // } else {
    //     Err(Box::new(std::io::Error::new(
    //         std::io::ErrorKind::NotFound,
    //         "Face not found",
    //     )))
    // }
    dice_faces
        .faces
        .get(roll_str)
        .map(|face| format!("{}", face))
        .ok_or_else(|| "Face not found.".into())
}
