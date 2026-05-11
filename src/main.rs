use std::sync::{Arc, Mutex};

use anyhow::Result;
use clap::{Parser, Subcommand};

use brace::build::SiteGenerator;
use brace::config::SiteConfig;

#[derive(Parser)]
#[command(name = "brace", about = "Brace Industries Static Site Generator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the site
    Build,
    /// Start dev server with file watching
    Serve {
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Create a new draft post
    New {
        /// Post title
        title: String,
    },
    /// Publish a draft
    Publish {
        /// Filename of the draft (e.g., my-post.md)
        filename: String,
    },
    /// Generate syntax highlighting CSS
    SyntaxCss {
        /// Theme name (e.g., base16-ocean.dark)
        #[arg(default_value = "base16-ocean.dark")]
        theme: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = SiteConfig::from_env();

    match cli.command {
        None | Some(Commands::Build) => {
            let mut generator = SiteGenerator::new(config)?;
            generator.build()?;
        }
        Some(Commands::Serve { port }) => {
            let config_clone = config.clone();
            let generator = SiteGenerator::new(config)?;
            let generator = Arc::new(Mutex::new(generator));
            brace::serve::serve(generator, config_clone, port)?;
        }
        Some(Commands::New { title }) => {
            brace::new::new_post(&config, &title)?;
        }
        Some(Commands::Publish { filename }) => {
            brace::publish::publish(&config, &filename)?;
            // Rebuild after publishing
            println!("\nRebuilding site...");
            let mut generator = SiteGenerator::new(config)?;
            generator.build()?;
        }
        Some(Commands::SyntaxCss { theme }) => {
            let css = brace::highlight::generate_syntax_css(&theme)?;
            let output = config.static_dir.join("syntax.css");
            std::fs::write(&output, css)?;
            println!("\u{2713} Generated syntax CSS: {}", output.display());
        }
    }

    Ok(())
}
