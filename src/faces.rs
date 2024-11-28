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

pub fn roll_dice(arg: &FacesArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    if let Some(faces) = arg.faces.as_ref() {
        let roll = rng.gen_range(1..=*faces);
        let roll_str: &str = &format!("{}", roll);
        match arg.disable_ascii {
            Some(true) => {
                print_figlet(roll_str)?;
            }
            _ => {
                if *faces <= 6 {
                    print_ascii(roll_str)?;
                } else {
                    print_figlet(roll_str)?;
                }
            }
        }
    }
    Ok(())
}

fn print_figlet(roll_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let standard_font = figlet_rs::FIGfont::standard()?;
    let figure = standard_font.convert(roll_str);
    println!("{}", figure.unwrap());
    Ok(())
}

fn print_ascii(roll_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dice_faces = load_dice_faces("faces.json")?;
    if let Some(face) = dice_faces.faces.get(roll_str) {
        println!("{}", face)
    };
    Ok(())
}
