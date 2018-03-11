use route::page_title;
use rocket_contrib::Template;

#[derive(Serialize)]
struct LoginPage<'c> {
    title: &'c str,
}

impl<'c> LoginPage<'c> {
    pub fn new(title: &'c str) -> LoginPage {
        LoginPage { title }
    }
}

#[get("/login")]
fn login() -> Template {
    let title = page_title("Login");

    let context = LoginPage::new(&title[..]);
    Template::render("login", &context)
}
