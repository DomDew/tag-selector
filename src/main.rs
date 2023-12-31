use dialoguer::{theme::ColorfulTheme, Select};
use git2::{build::CheckoutBuilder, Repository};
use std::env;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;

    let args: Vec<String> = env::args().skip(1).collect();

    let pattern = args
        .iter()
        .find(|arg| arg.starts_with("--pattern="))
        .map(|arg| arg.trim_start_matches("--pattern="))
        .unwrap_or("*");

    let tag_names = repo.tag_names(Some(pattern))?;

    if tag_names.is_empty() {
        println!("No tags found in this repository");
        return Ok(());
    }

    let tags = tag_names
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

    repo.set_head_detached(tag.id())?;
    repo.checkout_head(Some(CheckoutBuilder::new().force()))?;

    println!("Successfully checked out tag: {}", tag_name);

    Ok(())
}
