use std::collections::HashMap;
use std::io::{self, Write};

use comrak::adapters::SyntaxHighlighterAdapter;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "syn-" };

pub struct SyntectAdapter {
    pub syntax_set: SyntaxSet,
}

impl Default for SyntectAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntectAdapter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
        }
    }

    fn find_syntax(&self, lang: Option<&str>) -> &SyntaxReference {
        lang.and_then(|l| self.syntax_set.find_syntax_by_token(l))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }
}

impl SyntaxHighlighterAdapter for SyntectAdapter {
    fn write_highlighted(
        &self,
        output: &mut dyn Write,
        lang: Option<&str>,
        code: &str,
    ) -> io::Result<()> {
        let syntax = self.find_syntax(lang);
        let mut generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &self.syntax_set, CLASS_STYLE);

        for line in LinesWithEndings::from(code) {
            generator
                .parse_html_for_line_which_includes_newline(line)
                .map_err(io::Error::other)?;
        }

        output.write_all(generator.finalize().as_bytes())
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<String, String>,
    ) -> io::Result<()> {
        write!(output, "<pre")?;
        for (k, v) in &attributes {
            write!(output, " {}=\"{}\"", k, v)?;
        }
        write!(output, "><code>")
    }

    fn write_code_tag(
        &self,
        _output: &mut dyn Write,
        _attributes: HashMap<String, String>,
    ) -> io::Result<()> {
        // Handled in write_pre_tag
        Ok(())
    }
}

/// Generate a syntax highlighting CSS file from a syntect theme.
pub fn generate_syntax_css(theme_name: &str) -> anyhow::Result<String> {
    use syntect::highlighting::ThemeSet;
    use syntect::html::css_for_theme_with_class_style;

    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes.get(theme_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Theme '{}' not found. Available: {:?}",
            theme_name,
            theme_set.themes.keys().collect::<Vec<_>>()
        )
    })?;

    let css = css_for_theme_with_class_style(theme, CLASS_STYLE)?;
    Ok(css)
}
