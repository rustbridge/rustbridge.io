use rocket::response::NamedFile;
use rocket_contrib::Template;

#[derive(Serialize)]
struct DashBoard<'d> {
  title: &'d  str,
}

#[get("/dashboard")]
pub fn dashboard() -> Template {
  let title = super::page_title("DashBoard");
  let context = DashBoard { title: &title };

  Template::render("dashboard", &context)
}
