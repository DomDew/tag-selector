use dialoguer::{theme::ColorfulTheme, Select};
use git2::{build::CheckoutBuilder, CheckoutNotificationType, Repository};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;

    let tag_names = repo.tag_names(Some("*"))?;

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

    let mut builder = CheckoutBuilder::new();
    let checkout_opts = builder
        .conflict_style_merge(true)
        .notify_on(CheckoutNotificationType::CONFLICT);

    repo.checkout_tree(&tag, Some(checkout_opts))?;

    println!("Successfully checked out tag: {}", tag_name);

    Ok(())
}
