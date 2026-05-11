use std::fs;

use anyhow::{Context, Result};
use chrono::Local;

use crate::config::SiteConfig;

pub fn new_post(config: &SiteConfig, title: &str) -> Result<()> {
    // Generate slug from title
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    // Ensure drafts directory exists
    fs::create_dir_all(&config.drafts_dir)
        .context("Failed to create drafts directory")?;

    let filename = format!("{}.md", slug);
    let path = config.drafts_dir.join(&filename);

    if path.exists() {
        anyhow::bail!("Draft '{}' already exists at {}", filename, path.display());
    }

    let date = Local::now().format("%Y-%m-%d").to_string();
    let content = format!(
        "---\ntitle: \"{}\"\ndate: {}\npublished: true\n---\n\n",
        title.replace('"', "\\\""),
        date
    );

    fs::write(&path, content)?;
    println!("\u{2713} Created draft: {}", path.display());

    Ok(())
}
