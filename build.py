#!/usr/bin/env python3
"""
Brace Industries Static Site Generator
A simple Python-based static site generator for personal blog and project showcase.
"""

import os
import shutil
import argparse
from pathlib import Path
from datetime import datetime
import http.server
import socketserver
import markdown
import yaml
from jinja2 import Environment, FileSystemLoader


# Configuration
CONTENT_DIR = Path('content')
POSTS_DIR = CONTENT_DIR / 'posts'
DRAFTS_DIR = CONTENT_DIR / 'drafts'
TEMPLATES_DIR = Path('templates')
STATIC_DIR = Path('static')
OUTPUT_DIR = Path('docs')

# Base URL configuration
# - For local development with --serve: use empty string ""
# - For custom domain: use empty string "" (works with root domain)
# - For GitHub Pages subdirectory: use "/repo-name" (e.g., "/brace-industries")
BASE_URL = os.environ.get('BASE_URL', '')

# Custom domain for GitHub Pages (creates CNAME file)
# Set to empty string to disable CNAME generation
CUSTOM_DOMAIN = os.environ.get('CUSTOM_DOMAIN', 'braceindustries.com')


class Post:
    """Represents a blog post with metadata and content."""

    def __init__(self, filepath):
        self.filepath = Path(filepath)
        self.slug = self.filepath.stem
        self.metadata = {}
        self.content = ''
        self.html_content = ''
        self._parse()

    def _parse(self):
        """Parse markdown file with front-matter."""
        with open(self.filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        # Parse front-matter
        if content.startswith('---'):
            try:
                _, front_matter, body = content.split('---', 2)
                self.metadata = yaml.safe_load(front_matter) or {}
                self.content = body.strip()
            except ValueError:
                self.content = content
        else:
            self.content = content

        # Convert markdown to HTML
        md = markdown.Markdown(extensions=['extra', 'codehilite', 'meta'])
        self.html_content = md.convert(self.content)

        # Set default metadata
        if 'published' not in self.metadata:
            self.metadata['published'] = True
        if 'date' not in self.metadata:
            self.metadata['date'] = datetime.now()
        elif isinstance(self.metadata['date'], str):
            self.metadata['date'] = datetime.strptime(self.metadata['date'], '%Y-%m-%d')

    @property
    def title(self):
        return self.metadata.get('title', self.slug.replace('-', ' ').title())

    @property
    def date(self):
        return self.metadata.get('date')

    @property
    def published(self):
        return self.metadata.get('published', True)

    @property
    def url(self):
        return f'{BASE_URL}/writing/{self.slug}/'


class SiteGenerator:
    """Main site generator class."""

    def __init__(self):
        self.env = Environment(loader=FileSystemLoader(str(TEMPLATES_DIR)))
        self.posts = []

    def load_posts(self):
        """Load all published posts."""
        self.posts = []

        # Load from posts directory
        if POSTS_DIR.exists():
            for filepath in POSTS_DIR.glob('*.md'):
                post = Post(filepath)
                if post.published:
                    self.posts.append(post)

        # Sort by date, newest first
        self.posts.sort(key=lambda p: p.date, reverse=True)

    def build(self):
        """Build the entire site."""
        print("Building Brace Industries site...")

        # Clean output directory
        if OUTPUT_DIR.exists():
            shutil.rmtree(OUTPUT_DIR)
        OUTPUT_DIR.mkdir()

        # Create CNAME file for custom domain
        if CUSTOM_DOMAIN:
            cname_file = OUTPUT_DIR / 'CNAME'
            cname_file.write_text(CUSTOM_DOMAIN, encoding='utf-8')
            print(f"  Generated: {cname_file}")

        # Copy static files
        if STATIC_DIR.exists():
            shutil.copytree(STATIC_DIR, OUTPUT_DIR / 'static')

        # Load posts
        self.load_posts()

        # Generate pages
        self._generate_home()
        self._generate_writing_index()
        self._generate_posts()
        self._generate_about()
        self._generate_projects()
        self._generate_contact()
        self._generate_rss()

        print(f"✓ Built {len(self.posts)} posts")
        print(f"✓ Site generated in {OUTPUT_DIR}/")

    def _generate_home(self):
        """Generate home page."""
        template = self.env.get_template('home.html')
        recent_posts = self.posts[:5]  # Show 5 most recent

        html = template.render(posts=recent_posts, base_url=BASE_URL)

        output_file = OUTPUT_DIR / 'index.html'
        output_file.write_text(html, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def _generate_writing_index(self):
        """Generate writing index page."""
        template = self.env.get_template('writing.html')

        html = template.render(posts=self.posts, base_url=BASE_URL)

        writing_dir = OUTPUT_DIR / 'writing'
        writing_dir.mkdir(exist_ok=True)
        output_file = writing_dir / 'index.html'
        output_file.write_text(html, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def _generate_posts(self):
        """Generate individual post pages."""
        template = self.env.get_template('post.html')

        for post in self.posts:
            html = template.render(post=post, base_url=BASE_URL)

            post_dir = OUTPUT_DIR / 'writing' / post.slug
            post_dir.mkdir(parents=True, exist_ok=True)
            output_file = post_dir / 'index.html'
            output_file.write_text(html, encoding='utf-8')
            print(f"  Generated: {output_file}")

    def _generate_about(self):
        """Generate about page."""
        template = self.env.get_template('about.html')
        about_file = CONTENT_DIR / 'about.md'

        content = ''
        if about_file.exists():
            with open(about_file, 'r', encoding='utf-8') as f:
                raw_content = f.read()

            # Parse front-matter if present
            if raw_content.startswith('---'):
                try:
                    _, _, body = raw_content.split('---', 2)
                    raw_content = body.strip()
                except ValueError:
                    pass

            # Convert markdown to HTML
            md = markdown.Markdown(extensions=['extra', 'codehilite', 'meta'])
            content = md.convert(raw_content)

        html = template.render(content=content, base_url=BASE_URL)

        output_file = OUTPUT_DIR / 'about.html'
        output_file.write_text(html, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def _generate_projects(self):
        """Generate projects page."""
        template = self.env.get_template('projects.html')
        projects_file = CONTENT_DIR / 'projects.yaml'

        projects = []
        if projects_file.exists():
            with open(projects_file, 'r', encoding='utf-8') as f:
                projects = yaml.safe_load(f) or []

        html = template.render(projects=projects, base_url=BASE_URL)

        output_file = OUTPUT_DIR / 'projects.html'
        output_file.write_text(html, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def _generate_contact(self):
        """Generate contact page."""
        template = self.env.get_template('contact.html')

        html = template.render(base_url=BASE_URL)

        output_file = OUTPUT_DIR / 'contact.html'
        output_file.write_text(html, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def _generate_rss(self):
        """Generate RSS feed."""
        template = self.env.get_template('feed.xml')

        build_date = datetime.now().strftime('%a, %d %b %Y %H:%M:%S GMT')
        xml = template.render(posts=self.posts, build_date=build_date)

        output_file = OUTPUT_DIR / 'feed.xml'
        output_file.write_text(xml, encoding='utf-8')
        print(f"  Generated: {output_file}")

    def serve(self, port=3000):
        """Start local development server."""
        os.chdir(OUTPUT_DIR)

        handler = http.server.SimpleHTTPRequestHandler
        with socketserver.TCPServer(("", port), handler) as httpd:
            print(f"Serving site at http://localhost:{port}")
            print("Press Ctrl+C to stop")
            try:
                httpd.serve_forever()
            except KeyboardInterrupt:
                print("\nServer stopped.")

    def publish(self, filename):
        """Move draft to published posts."""
        draft_path = DRAFTS_DIR / filename

        if not draft_path.exists():
            print(f"Error: Draft '{filename}' not found in {DRAFTS_DIR}")
            return

        # Ensure posts directory exists
        POSTS_DIR.mkdir(parents=True, exist_ok=True)

        # Move file
        post_path = POSTS_DIR / filename
        shutil.move(str(draft_path), str(post_path))

        print(f"✓ Published: {filename}")
        print(f"  Moved from {draft_path} to {post_path}")

        # Update front-matter to set published: true
        post = Post(post_path)
        with open(post_path, 'r', encoding='utf-8') as f:
            content = f.read()

        if content.startswith('---'):
            parts = content.split('---', 2)
            if len(parts) >= 3:
                front_matter = yaml.safe_load(parts[1]) or {}
                front_matter['published'] = True

                # Add date if not present
                if 'date' not in front_matter:
                    front_matter['date'] = datetime.now().strftime('%Y-%m-%d')

                new_content = '---\n' + yaml.dump(front_matter, default_flow_style=False) + '---\n' + parts[2]
                with open(post_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)

        # Rebuild site
        print("\nRebuilding site...")
        self.build()


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(description='Brace Industries Static Site Generator')
    parser.add_argument('--serve', action='store_true', help='Start local dev server')
    parser.add_argument('--port', type=int, default=3000, help='Port for dev server (default: 3000)')
    parser.add_argument('--publish', metavar='FILENAME', help='Publish a draft post')
    parser.add_argument('--deploy', action='store_true', help='Show deployment instructions')

    args = parser.parse_args()

    generator = SiteGenerator()

    if args.publish:
        generator.publish(args.publish)
    elif args.serve:
        generator.build()
        generator.serve(args.port)
    elif args.deploy:
        print("""
Deployment to GitHub Pages:

1. Ensure your repository is initialized:
   git init (if not already done)

2. Add and commit your changes:
   git add .
   git commit -m "Build site"

3. Push to GitHub:
   git remote add origin https://github.com/USERNAME/brace-industries.git
   git branch -M main
   git push -u origin main

4. Enable GitHub Pages:
   - Go to your repository settings on GitHub
   - Navigate to "Pages" section
   - Set source to "Deploy from a branch"
   - Select branch: main
   - Select folder: /docs
   - Click Save

5. Your site will be live at:
   https://USERNAME.github.io/brace-industries/

Note: Replace USERNAME with your GitHub username.
""")
    else:
        generator.build()


if __name__ == '__main__':
    main()
