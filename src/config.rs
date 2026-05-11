use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub content_dir: PathBuf,
    pub posts_dir: PathBuf,
    pub drafts_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub static_dir: PathBuf,
    pub output_dir: PathBuf,
    pub base_url: String,
    pub custom_domain: Option<String>,
    pub site_title: String,
    pub site_description: String,
    pub site_url: String,
}

impl SiteConfig {
    pub fn from_env() -> Self {
        let base_url = env::var("BASE_URL").unwrap_or_default();
        let custom_domain = env::var("CUSTOM_DOMAIN")
            .unwrap_or_else(|_| "braceindustries.com".to_string());
        let custom_domain = if custom_domain.is_empty() {
            None
        } else {
            Some(custom_domain)
        };

        Self {
            content_dir: PathBuf::from("content"),
            posts_dir: PathBuf::from("content/posts"),
            drafts_dir: PathBuf::from("content/drafts"),
            templates_dir: PathBuf::from("templates"),
            static_dir: PathBuf::from("static"),
            output_dir: PathBuf::from("docs"),
            base_url,
            custom_domain,
            site_title: "Brace Industries".to_string(),
            site_description: "Thoughts on software, data, and building things".to_string(),
            site_url: "https://braceindustries.com".to_string(),
        }
    }
}
