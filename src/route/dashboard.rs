use failure::Error;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use route::{content_path, html_from_file, page_title, organizer::UserCookie};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct DashBoard<'d> {
    title: &'d str,
    parent: &'d str,
    content: String,
}

impl<'d> DashBoard<'d> {
    pub fn new<S: Into<String>>(title: &'d str, parent: &'d str, data: S) -> DashBoard<'d> {
        DashBoard {
            title,
            parent,
            content: data.into(),
        }
    }
}

fn render_dashboard(title: &str, content: PathBuf) -> Template {
    let template = || -> Result<Template, Error> {
        let page_content = html_from_file(&content.as_path())?;

        let context = DashBoard::new(title, "dashboard", page_content);

        Ok(Template::render("dashboard_content", &context))
    }().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    });

    template
}

fn home() -> Template {
    let title = page_title("DashBoard");
    let content = content_path("dashboard_activity.md");

    render_dashboard(&title, content)
}

fn invites() -> Template {
    let title = page_title("Invites");
    let content = content_path("dashboard_activity.md");

    render_dashboard(&title, content)
}

fn workshops() -> Template {
    let title = page_title("Your Workshops");
    let content = content_path("dashboard_activity.md");

    render_dashboard(&title, content)
}

fn create_workshop() -> Template {
    let title = page_title("Create Workshop");
    let content = content_path("dashboard_activity.md");

    render_dashboard(&title, content)
}

#[get("/dashboard/<page..>")]
pub fn dashboard(_user: UserCookie, page: PathBuf) -> Template {
    match &page.to_string_lossy().into_owned()[..] {
        "home" => home(),
        "workshops" => workshops(),
        "invites" => invites(),
        "create_workshop" => create_workshop(),
        _ => home(),
    }
}

#[get("/dashboard/<page..>", rank = 2)]
pub fn unauthenticated_dashboard(page: PathBuf) -> Redirect {
    Redirect::to("/login")
}
