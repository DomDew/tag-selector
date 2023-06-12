use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let tags = repo
        .tag_names(Some("*"))
        .expect("Failed to get tag names")
        .iter()
        .filter_map(|t| t.map(|s| s.to_string()))
        .collect::<Vec<String>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Git tag")
        .items(&tags)
        .default(0)
        .interact();

    let selected_tag = tags.get(selection.unwrap()).unwrap();

    println!("Selected tag: {}", selected_tag);
}
