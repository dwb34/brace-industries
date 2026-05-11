use std::fs;

use anyhow::{Context, Result};
use chrono::Local;

use crate::config::SiteConfig;

pub fn publish(config: &SiteConfig, filename: &str) -> Result<()> {
    let draft_path = config.drafts_dir.join(filename);

    if !draft_path.exists() {
        anyhow::bail!(
            "Draft '{}' not found in {}",
            filename,
            config.drafts_dir.display()
        );
    }

    // Ensure posts directory exists
    fs::create_dir_all(&config.posts_dir)
        .context("Failed to create posts directory")?;

    let post_path = config.posts_dir.join(filename);

    // Read and update frontmatter
    let content = fs::read_to_string(&draft_path)?;
    let updated = update_frontmatter(&content)?;

    // Write to posts directory
    fs::write(&post_path, &updated)?;

    // Remove draft
    fs::remove_file(&draft_path)?;

    println!("\u{2713} Published: {}", filename);
    println!("  Moved from {} to {}", draft_path.display(), post_path.display());

    Ok(())
}

fn update_frontmatter(content: &str) -> Result<String> {
    if !content.starts_with("---") {
        return Ok(content.to_string());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Ok(content.to_string());
    }

    let mut frontmatter: serde_yml::Value = serde_yml::from_str(parts[1])
        .context("Failed to parse frontmatter")?;

    if let serde_yml::Value::Mapping(ref mut map) = frontmatter {
        map.insert(
            serde_yml::Value::String("published".to_string()),
            serde_yml::Value::Bool(true),
        );

        if !map.contains_key(serde_yml::Value::String("date".to_string())) {
            let date = Local::now().format("%Y-%m-%d").to_string();
            map.insert(
                serde_yml::Value::String("date".to_string()),
                serde_yml::Value::String(date),
            );
        }
    }

    let yaml = serde_yml::to_string(&frontmatter)?;
    Ok(format!("---\n{}---\n{}", yaml, parts[2]))
}
