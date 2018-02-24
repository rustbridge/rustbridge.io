#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate comrak;

#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::fs::File;
use std::io::Read;
use rocket::http::Status;
use comrak::{markdown_to_html, ComrakOptions};
use std::path::PathBuf;
use std::path::Path;


#[derive(Serialize)]
struct TemplateContext {
    title: String,
    parent: String,
    data: String,
    res: String
}

fn get_html_from_file(path: String) -> String {
    println!("{}", path);
    let mut file = File::open(path).expect("failed to open path");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("failed to read");

    let html = markdown_to_html(&content, &ComrakOptions::default());
    return html;
}

#[get("/")]
fn index() -> Template {
    
    let title = "rust bridge".to_string();
    let about = get_html_from_file("data/about.md".to_string());
    let res = get_html_from_file("data/resources.md".to_string());

    let context = TemplateContext {
        title,
        data: about,
        res,
        parent: "layout".to_string(),
    };

    Template::render("page".to_string(), &context)
}

#[get("/<page>")]
fn page(page: String) -> Template {
    let title = format!("rust bridge - {}", page).to_string();
    let markdown_path = format!("data/{}.md", page);
    let data = get_html_from_file(markdown_path.to_string());
    let res = get_html_from_file("data/resources.md".to_string());

    let context = TemplateContext {
        title,
        data, 
        res,
        parent: "layout".to_string()
    };
    Template::render("page".to_string(), &context) 
}

#[get("/static/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index,files, page])
        .launch();

}
