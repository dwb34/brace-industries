use std::fs;

use anyhow::{Context, Result};
use chrono::Utc;
use tera::Tera;
use walkdir::WalkDir;

use crate::config::SiteConfig;
use crate::content::Post;
use crate::highlight::SyntectAdapter;
use crate::templates::load_templates;

pub struct SiteGenerator {
    config: SiteConfig,
    tera: Tera,
    highlighter: SyntectAdapter,
}

impl SiteGenerator {
    pub fn new(config: SiteConfig) -> Result<Self> {
        let tera = load_templates(&config)?;
        let highlighter = SyntectAdapter::new();
        Ok(Self {
            config,
            tera,
            highlighter,
        })
    }

    pub fn reload_templates(&mut self) -> Result<()> {
        self.tera = load_templates(&self.config)?;
        Ok(())
    }

    pub fn build(&mut self) -> Result<()> {
        println!("Building Brace Industries site...");

        // Reload templates on each build to pick up changes
        self.reload_templates()?;

        // Clean output directory
        if self.config.output_dir.exists() {
            fs::remove_dir_all(&self.config.output_dir)
                .context("Failed to clean output directory")?;
        }
        fs::create_dir_all(&self.config.output_dir)?;

        // Write CNAME
        if let Some(ref domain) = self.config.custom_domain {
            let cname_path = self.config.output_dir.join("CNAME");
            fs::write(&cname_path, domain)?;
            println!("  Generated: {}", cname_path.display());
        }

        // Copy static files
        if self.config.static_dir.exists() {
            let dest = self.config.output_dir.join("static");
            copy_dir_recursive(&self.config.static_dir, &dest)?;
        }

        // Load posts
        let posts = self.load_posts()?;

        // Generate pages
        self.generate_home(&posts)?;
        self.generate_writing_index(&posts)?;
        self.generate_posts(&posts)?;
        self.generate_contact()?;
        self.generate_rss(&posts)?;

        println!("\u{2713} Built {} posts", posts.len());
        println!("\u{2713} Site generated in {}/", self.config.output_dir.display());

        Ok(())
    }

    fn load_posts(&self) -> Result<Vec<Post>> {
        let mut posts = Vec::new();

        if self.config.posts_dir.exists() {
            for entry in fs::read_dir(&self.config.posts_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "md") {
                    let post = Post::from_file(&path, &self.config, &self.highlighter)?;
                    if post.published {
                        posts.push(post);
                    }
                }
            }
        }

        posts.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(posts)
    }

    fn generate_home(&self, posts: &[Post]) -> Result<()> {
        let recent: Vec<_> = posts.iter().take(5).collect();
        let mut context = tera::Context::new();
        context.insert("posts", &recent);
        context.insert("base_url", &self.config.base_url);

        let html = self.tera.render("home.html", &context)
            .context("Failed to render home.html")?;

        let output = self.config.output_dir.join("index.html");
        fs::write(&output, html)?;
        println!("  Generated: {}", output.display());
        Ok(())
    }

    fn generate_writing_index(&self, posts: &[Post]) -> Result<()> {
        let mut context = tera::Context::new();
        context.insert("posts", posts);
        context.insert("base_url", &self.config.base_url);

        let html = self.tera.render("writing.html", &context)
            .context("Failed to render writing.html")?;

        let dir = self.config.output_dir.join("writing");
        fs::create_dir_all(&dir)?;
        let output = dir.join("index.html");
        fs::write(&output, html)?;
        println!("  Generated: {}", output.display());
        Ok(())
    }

    fn generate_posts(&self, posts: &[Post]) -> Result<()> {
        for post in posts {
            let mut context = tera::Context::new();
            context.insert("post", post);
            context.insert("base_url", &self.config.base_url);

            let html = self.tera.render("post.html", &context)
                .with_context(|| format!("Failed to render post: {}", post.slug))?;

            let dir = self.config.output_dir.join("writing").join(&post.slug);
            fs::create_dir_all(&dir)?;
            let output = dir.join("index.html");
            fs::write(&output, html)?;
            println!("  Generated: {}", output.display());
        }
        Ok(())
    }

    fn generate_contact(&self) -> Result<()> {
        let mut context = tera::Context::new();
        context.insert("base_url", &self.config.base_url);

        let html = self.tera.render("contact.html", &context)
            .context("Failed to render contact.html")?;

        let output = self.config.output_dir.join("contact.html");
        fs::write(&output, html)?;
        println!("  Generated: {}", output.display());
        Ok(())
    }

    fn generate_rss(&self, posts: &[Post]) -> Result<()> {
        let mut context = tera::Context::new();
        context.insert("posts", posts);
        let build_date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        context.insert("build_date", &build_date);

        let xml = self.tera.render("feed.xml", &context)
            .context("Failed to render feed.xml")?;

        let output = self.config.output_dir.join("feed.xml");
        fs::write(&output, xml)?;
        println!("  Generated: {}", output.display());
        Ok(())
    }
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
