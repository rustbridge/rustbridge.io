pub mod organizer;

use failure::ResultExt;
use failure::Error;

use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs;

use comrak::{markdown_to_html, ComrakOptions};

use rocket::response::NamedFile;
use rocket_contrib::Template;

fn html_from_file(path: &Path) -> Result<String, Error> {
    let mut file = fs::File::open(&path)
        .with_context(|e| format!("Failed to open file: `{}`\n => {}", &path.display(), e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .with_context(|e| format!("Failed to read file: `{}`\n => {}", &path.display(), e))?;

    Ok(markdown_to_html(&content[..], &ComrakOptions::default()))
}

#[derive(Serialize)]
struct Page<'c> {
    title: &'c str,
    parent: &'c str,
    data: String,
    sidebar: String,
}

impl<'c> Page<'c> {
    pub fn new(title: &'c str, parent: &'c str, data: String, sidebar: String) -> Page<'c> {
        Page {
            title,
            parent,
            data,
            sidebar,
        }
    }
}

fn render_page(title: &str, content: PathBuf, sidebar: PathBuf) -> Template {
    let template = || -> Result<Template, Error> {
        let page_content = html_from_file(&content.as_path())?;
        let sidebar_content = html_from_file(&sidebar.as_path())?;

        let context = Page::new(title, "layout", page_content, sidebar_content);

        Ok(Template::render("page", &context))
    }()
        .unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    });

    template
}

pub fn page_title(current_page: &str) -> String {
    format!("RustBridge - {}", current_page)
}

pub fn content_path(file: &str) -> PathBuf {
    PathBuf::from("data").join(file)
}

#[get("/static/<asset..>", rank = 1)]
pub fn static_asset(asset: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(asset)).ok()
}

#[get("/")]
pub fn about() -> Template {
    let title = page_title("About");
    let content = content_path("about.md");
    let sidebar = content_path("workshops.md");

    render_page(&title[..], content, sidebar)
}

#[get("/learn")]
pub fn learn() -> Template {
    let title = page_title("Learn");
    let content = content_path("learn.md");
    let sidebar = content_path("resources.md");

    render_page(&title[..], content, sidebar)
}

#[get("/volunteer")]
pub fn volunteer() -> Template {
    let title = page_title("Volunteer");
    let content = content_path("volunteer.md");
    let sidebar = content_path("resources.md");

    render_page(&title[..], content, sidebar)
}
