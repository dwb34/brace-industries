use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use tiny_http::{Header, Response, Server};

use crate::build::SiteGenerator;
use crate::config::SiteConfig;

pub fn serve(generator: Arc<Mutex<SiteGenerator>>, config: SiteConfig, port: u16) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    // Initial build
    generator.lock().unwrap_or_else(|e| e.into_inner()).build()?;

    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to start server on {}: {}", addr, e))?;
    let server = Arc::new(server);

    println!("Serving at http://localhost:{}", port);
    println!("Watching for changes... (press Ctrl+C to stop)");

    // HTTP server thread
    let server_clone = Arc::clone(&server);
    let output_dir = config.output_dir.clone();
    let running_http = Arc::clone(&running);
    let http_thread = thread::spawn(move || {
        while running_http.load(Ordering::Relaxed) {
            match server_clone.recv_timeout(Duration::from_millis(500)) {
                Ok(Some(request)) => {
                    let url_path = request.url().to_string();
                    let file_path = resolve_path(&output_dir, &url_path);

                    match fs::read(&file_path) {
                        Ok(contents) => {
                            let content_type = mime_type(&file_path);
                            let header =
                                Header::from_bytes("Content-Type", content_type).unwrap();
                            let response =
                                Response::from_data(contents).with_header(header);
                            let _ = request.respond(response);
                        }
                        Err(_) => {
                            let response =
                                Response::from_string("404 Not Found").with_status_code(404);
                            let _ = request.respond(response);
                        }
                    }
                }
                Ok(None) => {} // timeout, loop back
                Err(_) => break,
            }
        }
    });

    // File watcher thread
    let gen_clone = Arc::clone(&generator);
    let running_watch = Arc::clone(&running);
    let watch_content = config.content_dir.clone();
    let watch_templates = config.templates_dir.clone();
    let watch_static = config.static_dir.clone();

    let (tx, rx) = std::sync::mpsc::channel::<DebounceEventResult>();
    let mut debouncer = new_debouncer(Duration::from_millis(500), tx)?;

    debouncer
        .watcher()
        .watch(&watch_content, notify::RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(&watch_templates, notify::RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(&watch_static, notify::RecursiveMode::Recursive)?;

    let watcher_thread = thread::spawn(move || {
        while running_watch.load(Ordering::Relaxed) {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(_events)) => {
                    println!("\nChange detected, rebuilding...");
                    match gen_clone.lock().unwrap_or_else(|e| e.into_inner()).build() {
                        Ok(_) => println!("Rebuild complete."),
                        Err(e) => eprintln!("Rebuild failed: {}", e),
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Watch error: {:?}", e);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(_) => break,
            }
        }
    });

    // Block until Ctrl+C
    let running_main = Arc::clone(&running);
    ctrlc::set_handler(move || {
        running_main.store(false, Ordering::Relaxed);
    })
    .ok();

    // Wait for shutdown signal (polling since ctrlc handler sets the flag)
    while running.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(200));
    }

    println!("\nShutting down...");
    server.unblock();
    http_thread.join().ok();
    watcher_thread.join().ok();

    Ok(())
}

fn resolve_path(output_dir: &Path, url_path: &str) -> PathBuf {
    // Strip query string
    let url_path = url_path.split('?').next().unwrap_or("/");
    let cleaned = url_path.trim_start_matches('/');
    let path = if cleaned.is_empty() {
        output_dir.join("index.html")
    } else {
        output_dir.join(cleaned)
    };

    // Prevent path traversal — resolved path must stay within output_dir
    if let (Ok(canonical_output), Ok(canonical_path)) =
        (output_dir.canonicalize(), path.canonicalize())
    {
        if !canonical_path.starts_with(&canonical_output) {
            return output_dir.join("__invalid__"); // will 404
        }
    }

    if path.is_dir() {
        path.join("index.html")
    } else if !path.exists() && path.extension().is_none() {
        let with_html = path.with_extension("html");
        if with_html.exists() {
            with_html
        } else {
            path
        }
    } else {
        path
    }
}

fn mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript",
        Some("xml") => "application/xml; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}
