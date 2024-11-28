#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::roll_dice;

pub fn App() -> Element {
    rsx! {
        div {
            h1 { "Roll dice {e}" }
            button { onclick: move |_|{
    let defaults = crate::FacesArgs {
        faces: Some(6),
        disable_ascii: Some(false),
    };
    let e = roll_dice(&defaults);
     "roll the die";

            } }
        }
    }
}
