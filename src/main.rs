use std::format;

use console::Term;
use console::style;
use dialoguer::{
    FuzzySelect,
    Input,
    theme::ColorfulTheme,
};
use itertools::Itertools;

mod option;

fn main() -> Result<(), std::io::Error> {
    let choices = option::load_options().unwrap();


    let term = Term::stdout();
    term.set_title("CLI Test Application");
    term.clear_screen()?;

    let languages: Vec<String> = choices
        .iter()
        .map(|x| String::from(&x.language))
        .unique()
        .collect();

    let chosen = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("{}", style("Please choose a language or platform").bold().underlined()))
        .items(&languages)
        .interact()?;

    let templates: Vec<&option::RepoSource> = choices.iter().filter(|x| x.language == languages[chosen]).collect();

    let options: Vec<String> =
        templates
            .iter()
            .map(|x| format!("{}\n    {}", String::from(&x.title), String::from(&x.description)))
            .collect();

    let selected_template = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!("{}", style("Please choose a template").bold().underlined()))
        .items(&options)
        .interact()?;

    let project_name : String = Input::new()
        .with_prompt(format!("{}", style("Where should the project be created?").bold().underlined()))
        .interact_text()?;

    let url = project_name.replace(" ", "-").to_lowercase();

    templates[selected_template].create(&url);


    Ok(())
}
