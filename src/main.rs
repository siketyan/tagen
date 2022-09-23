use clap::Parser;
use tagen::tagen;

/// Generates lots of tags by version and variants.
#[derive(Debug, clap::Parser)]
struct Cli {
    /// Version of the something to tag.
    version: String,

    /// Variants to append to the tags.
    variants: Vec<String>,
}

fn main() {
    let Cli { version, variants } = Cli::parse();

    tagen(
        version,
        &variants.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    )
    .into_iter()
    .for_each(|tag| println!("{}", tag))
}
