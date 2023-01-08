use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::ProgressBar;

fn main() {
    let selection = &["Download game", "Exit"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .default(0)
            .items(selection)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let url: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Game URL")
                    .interact_text()
                    .unwrap();

                let progress = ProgressBar::new_spinner();
                progress.enable_steady_tick(Duration::from_millis(120));
                progress.set_message("Dowloading...");
                download(&url, "game-source");
                progress.finish_with_message("Wrote to file");
            }
            1 => return,

            _ => panic!("index out of bounds"),
        }
    }
}

fn download(url: &str, name: &str) {
    let response = minreq::get(url).send().expect("download url");
    let response = response.as_bytes();
    std::fs::write(name, response).expect("write html to file");
}
