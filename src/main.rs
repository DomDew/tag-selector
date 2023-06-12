use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let tags = repo.tag_names(Some("*")).unwrap();

    println!("Tags: {}", tags.len());

    println!("Hello, world!");
}
