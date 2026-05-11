use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use serde::{Deserialize, Serialize};

use crate::config::SiteConfig;
use crate::highlight::SyntectAdapter;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: Option<String>,
    #[serde(default = "default_published")]
    published: bool,
    date: Option<NaiveDate>,
}

fn default_published() -> bool {
    true
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    pub published: bool,
    pub html_content: String,
    pub url: String,
    pub date_iso: String,
    pub date_short: String,
    pub date_long: String,
    pub date_rfc2822: String,
}

impl Post {
    pub fn from_file(path: &Path, config: &SiteConfig, highlighter: &SyntectAdapter) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let slug = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let (frontmatter, body) = parse_frontmatter(&content)?;

        let html_content = render_markdown(&body, highlighter);

        let title = frontmatter
            .title
            .unwrap_or_else(|| slug.replace('-', " "));

        let date = frontmatter
            .date
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        let url = format!("{}/writing/{}/", config.base_url, slug);

        let date_iso = date.format("%Y-%m-%d").to_string();
        let date_short = date.format("%b %d, %Y").to_string();
        let date_long = date.format("%B %d, %Y").to_string();
        let date_rfc2822 = date.format("%a, %d %b %Y 00:00:00 GMT").to_string();

        Ok(Self {
            slug,
            title,
            date,
            published: frontmatter.published,
            html_content,
            url,
            date_iso,
            date_short,
            date_long,
            date_rfc2822,
        })
    }
}

fn parse_frontmatter(content: &str) -> Result<(Frontmatter, String)> {
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let yaml_str = parts[1];
            let body = parts[2].trim().to_string();
            let frontmatter: Frontmatter = serde_yml::from_str(yaml_str)
                .context("Failed to parse frontmatter YAML")?;
            return Ok((frontmatter, body));
        }
    }

    Ok((
        Frontmatter {
            title: None,
            published: true,
            date: None,
        },
        content.to_string(),
    ))
}

fn render_markdown(source: &str, highlighter: &SyntectAdapter) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.header_ids = Some(String::new());
    options.render.unsafe_ = true;

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(highlighter);

    markdown_to_html_with_plugins(source, &options, &plugins)
}
