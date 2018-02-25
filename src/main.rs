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
use comrak::{markdown_to_html, ComrakOptions};
use std::path::PathBuf;
use std::path::Path;
use rocket::request::Form;

mod form;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    parent: String,
    data: String,
    sidebar: String
}

#[derive(Serialize)]
struct Context {

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
    let sidebar = get_html_from_file("data/workshops.md".to_string());

    let context = TemplateContext {
        title,
        data: about,
        sidebar,
        parent: "layout".to_string(),
    };

    Template::render("page".to_string(), &context)
}

#[get("/<page>", rank=1)]
fn page(page: String) -> Template {
    let title = format!("rust bridge - {}", page).to_string();
    let markdown_path = format!("data/{}.md", page);
    let data = get_html_from_file(markdown_path.to_string());

    println!("{} hmm page is ::: ", page);

    let sidebar = if page == "about" {
            get_html_from_file("data/workshops.md".to_string())
        } else {
            get_html_from_file("data/resources.md".to_string())
        };

    println!("{} WOHOHOH", sidebar);

    let context = TemplateContext {
        title,
        data, 
        sidebar,
        parent: "layout".to_string()
    };
    Template::render("page".to_string(), &context) 
}

#[get("/forgot_username")] 
fn forgot_username() -> Template {
    println!("We forgot username");
    let c = Context {};
    Template::render("reset_username".to_string(), &c) 
}

#[derive(Debug)]
struct Login {
    email: String,
    password: String
}

#[derive(Debug)]
struct Workshop {
    name: String,
    reg_link: String,
    desc: String,
    start_time: String,
    end_time: String,
    date: String
}

#[derive(Debug)]
struct ForgotUsername {
    email: String,
    login: bool, // user clicked login or not...fix this...
}

#[post("/reset_username", data="<form_data>")]
fn reset_username(form_data: Form<ForgotUsername>) -> Template {
    let c = Context {};
    if form_data.get().login {
        return Template::render("login".to_string(), &c) 
    } else {
        println!("We should reset usr password");
    }
    Template::render("reset_username".to_string(), &c) 
}

#[post("/reset_username", rank=2)] 
fn login_again() -> String {
    "heheh".to_string()
}


#[post("/login", data="<form_data>")]
fn login_submit(form_data: Form<Login>) -> Template {
    println!("{:?}", form_data.get());  
    let c = Context {};
    Template::render("post_workshop".to_string(), &c) 
}

#[get("/login")]
fn login() -> Template {
    let c = Context {};
    Template::render("login".to_string(), &c) 
}


#[get("/static/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/post_workshop", data="<form_data>")]
fn workshop_submit(form_data: Form<Workshop>) -> String {
    println!("{:?}", form_data.get());  
    "post_workshop".to_string()
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index,files,page,login, login_submit, forgot_username, reset_username, workshop_submit])
        .launch();

}
