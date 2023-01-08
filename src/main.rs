use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::ProgressBar;
use scraper::{Html, Selector};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

fn main() {
    let selection = &["Select game", "Download game", "Exit"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select action")
            .default(0)
            .items(selection)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let games = get_games();
                let game_index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select game")
                    .default(0)
                    .items(&games)
                    .interact()
                    .unwrap();

                let game = &games[game_index];
                let game_data =
                    std::fs::read(format!("data/games/{game}")).expect("read game file");
                game_menu(&String::from_utf8_lossy(&game_data));
            }
            1 => {
                let url: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Game URL")
                    .interact_text()
                    .unwrap();
                let filename: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Filename")
                    .with_initial_text("Game.html")
                    .interact_text()
                    .unwrap();

                let progress = ProgressBar::new_spinner();
                progress.enable_steady_tick(Duration::from_millis(120));
                progress.set_message("Dowloading...");
                download(&url, &filename);
                progress.finish_with_message("Wrote to file");
                eprintln!();
            }
            _ => return,
        }
        eprintln!();
    }
}

fn download(url: &str, name: &str) {
    let response = minreq::get(url).send().expect("download url");
    let response = response.as_bytes();
    std::fs::create_dir_all("data/games").expect("create data/games directory");
    std::fs::write(format!("data/games/{name}"), response).expect("write html to file");
}

fn get_games() -> Vec<String> {
    std::fs::read_dir("data/games")
        .expect("read data/games dir")
        .map(|x| x.expect("read data/games dir entry"))
        .map(|d| {
            d.file_name()
                .to_str()
                .expect("filename should be unicode")
                .to_string()
        })
        .collect()
}

fn game_menu(data: &str) {
    let doc = Html::parse_document(data);

    loop {
        let options = ["View Script", "View Style", "Back"];

        let game_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match game_index {
            0 => {
                let script_selector = Selector::parse("script").unwrap();
                let scripts = doc
                    .select(&script_selector)
                    .filter_map(|r| r.value().attr("id"))
                    .collect::<Vec<_>>();

                let script_index = Select::with_theme(&ColorfulTheme::default())
                    .items(&scripts)
                    .default(0)
                    .interact()
                    .unwrap();

                let found_script = doc
                    .select(&script_selector)
                    .find(|r| r.value().attr("id") == Some(scripts[script_index]))
                    .unwrap();
                let inner = found_script.text().collect::<String>();

                print_highlighted(&inner, "js");
            }
            1 => {
                let style_selector = Selector::parse("style").unwrap();
                let styles = doc
                    .select(&style_selector)
                    .filter_map(|r| r.value().attr("id"))
                    .collect::<Vec<_>>();

                let style_index = Select::with_theme(&ColorfulTheme::default())
                    .items(&styles)
                    .default(0)
                    .interact()
                    .unwrap();

                let found_style = doc
                    .select(&style_selector)
                    .find(|r| r.value().attr("id") == Some(styles[style_index]))
                    .unwrap();
                let inner = found_style.text().collect::<String>();

                print_highlighted(&inner, "css");
            }
            _ => return,
        }
    }
}

fn print_highlighted(text: &str, ext: &str) {
    let themes = ThemeSet::load_defaults();
    let syntaxes = SyntaxSet::load_defaults_nonewlines();

    let theme = &themes.themes["base16-ocean.dark"];
    let syntax = syntaxes
        .find_syntax_by_extension(ext)
        .expect("cannot find given extension");

    let mut highlighter = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(text) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            highlighter.highlight_line(line, &syntaxes).unwrap();
        let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }

    // Clear the formatting
    println!("\x1b[0m");
}
