use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;

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
        .interact()?;

    let tag_name = tags.get(selection).ok_or("Invalid selection")?;

    let tag = repo.revparse_single(tag_name)?;
    repo.checkout_tree(&tag, None)?;

    println!("Successfully checked out tag: {}", tag_name);

    Ok(())
}
