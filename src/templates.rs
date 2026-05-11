use anyhow::{Context, Result};
use tera::Tera;

use crate::config::SiteConfig;

pub fn load_templates(config: &SiteConfig) -> Result<Tera> {
    let pattern = format!("{}/**/*", config.templates_dir.display());
    let mut tera = Tera::new(&pattern).context("Failed to load templates")?;
    // Only auto-escape .html files; XML feed handles its own escaping via | safe
    tera.autoescape_on(vec![".html", ".xml"]);
    Ok(tera)
}
