# Brace Industries Static Site Generator

A minimal, elegant static site generator for personal blogs and project showcases. Built with Python, featuring markdown parsing, Jinja2 templating, and a warm, professional design inspired by Living Roots Wine Co.

## Features

- 📝 Markdown-based content with YAML front-matter
- 🖼️ Image support in articles with automatic asset management
- 🎨 Clean, responsive design with warm terracotta and cream color palette
- 🎯 Custom favicon for professional branding
- 📁 Draft and published post management
- 🚀 GitHub Pages deployment ready
- 🛠️ Simple CLI for building, serving, and publishing
- 📱 Mobile-first responsive design

## Quick Start

### Installation

1. **Install uv** (if you haven't already):
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

2. **Clone the repository** (or ensure you're in the project directory):
   ```bash
   cd brace-industries
   ```

3. **Install dependencies**:
   ```bash
   uv sync
   ```

### Usage

#### Build the site
Generate static files in the `/docs` directory:
```bash
uv run python build.py
```

#### Run local development server
Build and serve the site locally at `http://localhost:3000`:
```bash
uv run python build.py --serve
```

Use a custom port:
```bash
uv run python build.py --serve --port 8080
```

#### Publish a draft
Move a draft from `/content/drafts/` to `/content/posts/` and rebuild:
```bash
uv run python build.py --publish my-post.md
```

#### Deployment instructions
View GitHub Pages deployment instructions:
```bash
uv run python build.py --deploy
```

## Project Structure

```
brace-industries/
├── build.py                 # Main build script
├── pyproject.toml          # Project config and dependencies
├── uv.lock                 # Locked dependencies (auto-generated)
├── README.md               # This file
├── .gitignore              # Git ignore patterns
│
├── content/
│   ├── drafts/            # Unpublished posts
│   └── posts/             # Published posts
│
├── templates/
│   ├── base.html          # Base template with nav/footer
│   ├── home.html          # Home page
│   ├── writing.html       # Writing index page
│   ├── post.html          # Individual post page
│   └── contact.html       # Contact page
│
├── static/
│   ├── style.css          # Main stylesheet
│   ├── favicon.svg        # Site favicon
│   └── images/            # Images for articles
│
└── docs/                  # Generated site (GitHub Pages)
```

## Writing Content

### Create a new post

1. **Create a markdown file** in `/content/posts/` or `/content/drafts/`:
   ```bash
   touch content/posts/my-first-post.md
   ```

2. **Add front-matter and content**:
   ```markdown
   ---
   title: My First Post
   date: 2025-10-25
   published: true
   ---

   # Welcome to my blog

   This is the content of my first post. You can use **markdown** syntax here.

   ## Subheading

   - Bullet points
   - Are supported
   - Too!
   ```

### Front-matter options

- `title`: Post title (defaults to filename if not provided)
- `date`: Publication date in YYYY-MM-DD format (defaults to current date)
- `published`: Boolean to control visibility (defaults to `true`)

### Draft workflow

1. **Create draft**:
   ```bash
   # Create file in drafts folder
   touch content/drafts/work-in-progress.md
   ```

2. **Edit and preview**:
   ```bash
   # Drafts won't appear in the built site
   uv run python build.py --serve
   ```

3. **Publish when ready**:
   ```bash
   uv run python build.py --publish work-in-progress.md
   ```

Alternatively, you can use `published: false` in the front-matter of posts in `/content/posts/` to keep them unpublished.

### Adding images to articles

You can easily embed images in your articles using standard markdown syntax:

1. **Add your image** to the `/static/images/` directory:
   ```bash
   # Copy your image file
   cp ~/my-photo.jpg static/images/
   ```

2. **Reference the image** in your markdown:
   ```markdown
   ![Alt text description](/static/images/my-photo.jpg)
   ```

3. **Supported formats**: JPG, PNG, SVG, GIF, and any web-compatible image format

**Example:**
```markdown
---
title: My Photo Blog Post
date: 2025-10-25
---

Check out this amazing diagram:

![Architecture Diagram](/static/images/architecture.svg)

The image above shows our system architecture.
```

**Best practices:**
- Use descriptive alt text for accessibility
- Optimize images for web (compress large files)
- Use SVG for diagrams and logos when possible
- Store all images in `/static/images/` for organization

## Customization

### Update contact information
Edit `templates/contact.html` and replace placeholder links with your actual contact details.

### Modify colors
Edit `static/style.css` and update the CSS variables at the top:
```css
:root {
    --color-terracotta: #C1665A;
    --color-cream: #F5F1E8;
    --color-charcoal: #2B2B2B;
    /* ... */
}
```

### Update site name
Replace "Brace Industries" in:
- `templates/base.html` (logo and title)
- `templates/home.html` (page title)
- Other template files as needed

### Customize the favicon
The site includes a simple SVG favicon (`/static/favicon.svg`) with a "B" on a terracotta background. To customize it:

1. **Replace the SVG file** with your own design:
   ```bash
   # Overwrite with your favicon
   cp ~/my-favicon.svg static/favicon.svg
   ```

2. **Or create a new one** by editing `/static/favicon.svg`:
   ```svg
   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
     <rect width="100" height="100" fill="#C1665A"/>
     <text x="50" y="70" font-family="serif" font-size="60"
           font-weight="bold" text-anchor="middle" fill="#F5F1E8">B</text>
   </svg>
   ```

3. **Use PNG or ICO format** (optional):
   - Save your favicon as `favicon.png` or `favicon.ico`
   - Update the link in `templates/base.html`:
     ```html
     <link rel="icon" type="image/png" href="/static/favicon.png">
     ```

## Deployment to GitHub Pages

1. **Build your site**:
   ```bash
   uv run python build.py
   ```

2. **Commit and push to GitHub**:
   ```bash
   git add .
   git commit -m "Build site"
   git push origin main
   ```

3. **Enable GitHub Pages**:
   - Go to your repository settings on GitHub
   - Navigate to "Pages" section
   - Set source to "Deploy from a branch"
   - Select branch: `main`
   - Select folder: `/docs`
   - Click Save

4. **Access your site**:
   ```
   https://USERNAME.github.io/brace-industries/
   ```

## Design Philosophy

The design is inspired by:
- **darioamodei.com**: Minimal, single-column layout with focus on content
- **Living Roots Wine Co**: Warm terracotta and cream color palette
- **Works in Progress**: Soft, warm background tones for readability

Typography emphasizes readability with clean serif fonts for body text and sans-serif for headings.

## Dependencies

- **Python 3.9+**
- **uv**: Fast Python package installer and resolver
- **markdown**: Markdown parsing with extensions
- **Jinja2**: Template engine
- **PyYAML**: YAML front-matter parsing

Dependencies are managed via `pyproject.toml` and automatically installed with `uv sync`.

## License

This project is open source. Feel free to use and modify for your own projects.

## Support

For issues or questions, please open an issue on GitHub.
