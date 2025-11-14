# Deploying to GitHub Pages with Cloudflare Domain

This guide walks you through deploying the Brace Industries site to GitHub Pages with a custom domain managed by Cloudflare.

## Prerequisites

- Python 3.9+ installed
- `uv` package manager installed
- GitHub account
- Custom domain registered with Cloudflare
- Git installed and configured

## Phase 1: Prepare Your Local Site (2-3 minutes)

### Step 1: Build the Site

Run the build script to generate the latest HTML in the `/docs` folder:

```bash
uv run python build.py
```

This will:
- Clean the `/docs` directory
- Copy static assets
- Generate HTML from your markdown posts
- Create the site structure ready for GitHub Pages

### Step 2: Review Changes

Check what files have been modified:

```bash
git status
```

You should see changes in `/docs` and any content updates you've made.

### Step 3: Commit Your Changes

Stage and commit all changes:

```bash
git add .
git commit -m "Build site for GitHub Pages deployment"
```

## Phase 2: GitHub Repository Setup (3-5 minutes)

### Step 4: Create GitHub Repository (if not already created)

1. Go to [github.com](https://github.com) and log in
2. Click the "+" icon in the top right → "New repository"
3. Repository settings:
   - **Name**: `brace-industries` (or use your domain name like `yourdomain-com`)
   - **Description**: "Personal website and blog"
   - **Visibility**: Public (required for free GitHub Pages)
   - **Do NOT initialize** with README, .gitignore, or license (your repo already exists locally)
4. Click "Create repository"

### Step 5: Connect Local Repo to GitHub

Add GitHub as a remote and push your code:

```bash
# Replace YOUR_USERNAME and REPO_NAME with your actual values
git remote add origin https://github.com/YOUR_USERNAME/REPO_NAME.git

# Push your code
git push -u origin main
```

### Step 6: Enable GitHub Pages

1. In your GitHub repository, click **Settings** (top navigation)
2. In the left sidebar, click **Pages**
3. Under "Build and deployment":
   - **Source**: Deploy from a branch
   - **Branch**: Select `main` from dropdown
   - **Folder**: Select `/docs` from dropdown
   - Click **Save**
4. GitHub will display a message: "Your site is ready to be published at https://YOUR_USERNAME.github.io/REPO_NAME/"

Wait 1-2 minutes for the initial deployment to complete. You'll see a green checkmark when it's live.

## Phase 3: Cloudflare DNS Configuration (5-10 minutes)

### Step 7: Configure DNS Records in Cloudflare

1. Log in to your [Cloudflare dashboard](https://dash.cloudflare.com/)
2. Select your domain
3. Click **DNS** in the left sidebar
4. Add the following DNS records:

#### For Root Domain (example.com)

Add four A records pointing to GitHub Pages servers:

| Type | Name | IPv4 Address | Proxy Status | TTL |
|------|------|--------------|--------------|-----|
| A | @ | 185.199.108.153 | DNS only (gray cloud) | Auto |
| A | @ | 185.199.109.153 | DNS only (gray cloud) | Auto |
| A | @ | 185.199.110.153 | DNS only (gray cloud) | Auto |
| A | @ | 185.199.111.153 | DNS only (gray cloud) | Auto |

**Important**: Set Proxy status to "DNS only" (gray cloud icon) initially. You can enable Cloudflare proxy (orange cloud) after confirming the setup works.

#### For WWW Subdomain

Add a CNAME record:

| Type | Name | Target | Proxy Status | TTL |
|------|------|--------|--------------|-----|
| CNAME | www | YOUR_USERNAME.github.io | DNS only (gray cloud) | Auto |

Replace `YOUR_USERNAME` with your actual GitHub username.

### Step 8: Configure Custom Domain in GitHub

1. Back in your GitHub repository, go to **Settings** → **Pages**
2. Under "Custom domain", enter your domain (e.g., `yourdomain.com`)
3. Click **Save**
4. GitHub will create a CNAME file in your `/docs` folder and check DNS configuration
5. Wait for the DNS check to complete (can take 10-60 minutes)
6. Once verified, check **Enforce HTTPS** (GitHub will automatically provision an SSL certificate)

### Step 9: Wait for DNS Propagation

- DNS changes can take 10 minutes to 48 hours to propagate worldwide
- Typically, Cloudflare DNS updates are fast (under 10 minutes)
- Test your domain by visiting it in a browser
- You can check DNS propagation at [whatsmydns.net](https://www.whatsmydns.net/)

## Phase 4: Optional - Automated Builds with GitHub Actions (5 minutes)

Currently, you need to manually run `build.py` before pushing changes. You can automate this with GitHub Actions.

### Step 10: Create GitHub Actions Workflow

Create the workflow file:

```bash
mkdir -p .github/workflows
```

Create `.github/workflows/build.yml` with this content:

```yaml
name: Build and Deploy

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Install uv
      uses: astral-sh/setup-uv@v2

    - name: Set up Python
      run: uv python install 3.11

    - name: Install dependencies
      run: uv sync

    - name: Build site
      run: uv run python build.py

    - name: Commit and push if changed
      run: |
        git config --global user.name 'GitHub Actions'
        git config --global user.email 'actions@github.com'
        git add docs/
        git diff --quiet && git diff --staged --quiet || (git commit -m "Auto-build site [skip ci]" && git push)
```

Commit and push this workflow:

```bash
git add .github/workflows/build.yml
git commit -m "Add GitHub Actions workflow for automated builds"
git push
```

Now, whenever you push changes to markdown files or templates, GitHub Actions will automatically rebuild the site.

## Future Workflow

### Making Content Updates

1. **Edit or add content**:
   - Add new posts to `/content/posts/`
   - Or move drafts from `/content/drafts/` using `uv run python build.py --publish FILENAME`

2. **Build locally** (if not using GitHub Actions):
   ```bash
   uv run python build.py
   ```

3. **Preview locally** (optional):
   ```bash
   uv run python build.py --serve
   # Visit http://localhost:3000
   ```

4. **Commit and push**:
   ```bash
   git add .
   git commit -m "Add new blog post"
   git push
   ```

5. **Site updates automatically** on GitHub Pages within 1-2 minutes

### With GitHub Actions (Automated)

If you set up the workflow in Step 10, you can skip the manual build:

1. **Edit content**
2. **Commit and push** (without building):
   ```bash
   git add .
   git commit -m "Add new blog post"
   git push
   ```
3. **GitHub Actions builds and deploys automatically**

## Troubleshooting

### Site Not Loading

- **DNS not propagated yet**: Wait up to 48 hours (usually much faster)
- **Check DNS records**: Verify A records and CNAME in Cloudflare
- **GitHub Pages status**: Check Settings → Pages for error messages
- **CNAME file**: Ensure `/docs/CNAME` exists with your domain name

### HTTPS Certificate Issues

- **Wait for provisioning**: Can take 10-60 minutes after DNS verification
- **Disable Cloudflare proxy**: Set DNS to "DNS only" (gray cloud) until HTTPS works
- **Check domain**: Make sure custom domain is saved in GitHub Pages settings

### Build Failures

- **Missing dependencies**: Run `uv sync` to install Python packages
- **Python version**: Ensure Python 3.9+ is installed
- **Check logs**: Look at build.py output for specific errors

### Custom Domain Not Working

- **CNAME file location**: Must be in `/docs/CNAME` (not root)
- **DNS settings**: Double-check A records point to correct GitHub IPs
- **WWW vs non-WWW**: Decide which is primary, redirect the other

### Cloudflare Proxy Issues

- **Orange cloud**: Can cause SSL/HTTPS issues initially
- **Solution**: Use "DNS only" (gray cloud) until everything works
- **After working**: Can enable Cloudflare proxy for additional features

## Useful Commands

```bash
# Build site
uv run python build.py

# Build and serve locally
uv run python build.py --serve

# Publish a draft
uv run python build.py --publish FILENAME

# Check git status
git status

# View deployment info
uv run python build.py --deploy

# Check GitHub Actions logs
# Visit: https://github.com/YOUR_USERNAME/REPO_NAME/actions
```

## Resources

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Cloudflare DNS Documentation](https://developers.cloudflare.com/dns/)
- [GitHub Pages Custom Domain Guide](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site)
- [uv Documentation](https://docs.astral.sh/uv/)

## Support

If you encounter issues:
1. Check the Troubleshooting section above
2. Review GitHub Pages deployment logs
3. Verify DNS settings in Cloudflare
4. Check GitHub Actions workflow runs (if enabled)

---

**Congratulations!** Your site should now be live at your custom domain with automatic HTTPS.
