pub mod dashboard;
pub mod organizer;

use failure::Error;
use failure::ResultExt;

use db;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use comrak::{markdown_to_html, ComrakOptions};
use model::workshop::Workshop as WorkshopModel;

use rocket::response::NamedFile;
use rocket_contrib::Template;

#[get("/static/<asset..>", rank = 1)]
pub fn static_asset(asset: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(asset)).ok()
}

#[get("/")]
pub fn about() -> Result<Template, Error> {
    let page_content = markdown(content_path("about.md").as_path())?;

    let context = json!({
      "title": page_title("About"),
      "parent": "main_page/layout",
      "sidebar": "main_page/workshops",
      "content": page_content,
      "items": upcoming_workshops(),
    });

    Ok(Template::render("main_page/page", &context))
}

#[get("/learn")]
pub fn learn() -> Result<Template, Error> {
    let page_content = markdown(content_path("learn.md").as_path())?;
    let sidebar = markdown(content_path("resources.md").as_path())?;

    let context = json!({
      "title": page_title("Learn"),
      "parent": "main_page/layout",
      "sidebar": "main_page/sidebar",
      "content": page_content,
      "sidebar_content": sidebar,
    });

    Ok(Template::render("main_page/page", &context))
}

#[get("/volunteer")]
pub fn volunteer() -> Result<Template, Error> {
    let page_content = markdown(content_path("volunteer.md").as_path())?;
    let sidebar = markdown(content_path("resources.md").as_path())?;

    let context = json!({
      "title": page_title("Volunteer"),
      "parent": "main_page/layout",
      "sidebar": "main_page/sidebar",
      "content": page_content,
      "sidebar_content": sidebar,
    });

    Ok(Template::render("main_page/page", &context))
}

fn upcoming_workshops() -> Vec<WorkshopModel> {
    use diesel::prelude::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();
    workshops
        .filter(private.eq(false))
        .get_results(&connection)
        .unwrap()
}

fn markdown(path: &Path) -> Result<String, Error> {
    let mut file = fs::File::open(path)
        .with_context(|e| format!("Failed to open file: `{}`\n => {}", &path.display(), e))?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .with_context(|e| format!("Failed to read file: `{}`\n => {}", &path.display(), e))?;

    Ok(markdown_to_html(&content[..], &ComrakOptions::default()))
}

pub fn page_title(current_page: &str) -> String {
    format!("RustBridge - {}", current_page)
}

pub fn content_path(file: &str) -> PathBuf {
    PathBuf::from("data").join(file)
}
