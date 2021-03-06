use route::page_title;
use form::login::Login;
use model::user::User;
use db;

use rocket_contrib::Template;
use rocket::request::{FlashMessage, Form, FromRequest, Outcome, Request};
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use rocket::outcome::IntoOutcome;

use data_encoding::HEXUPPER;
use ring::{digest, rand, pbkdf2};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn salt(username: &str) -> Result<Vec<u8>, ()> {
    let db_salt = db::salt_component().map_err(|_| ())?;
    let mut res = Vec::with_capacity(username.as_bytes().len() + db_salt.as_bytes().len());

    res.extend(db_salt.as_bytes());
    res.extend(username.as_bytes());

    Ok(res)
}

fn verify_password(email: &str, pw: &str, expected_pw_hash: &str) -> bool {
    let pw_salt = salt(email).unwrap();

    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(DIGEST_ALG, 100_000, &pw_salt, pw.as_bytes(), &mut to_store);

    HEXUPPER.encode(&to_store) == expected_pw_hash.to_string()
}

#[derive(Serialize)]
struct LoginPage<'c> {
    title: &'c str,
    flash: Option<&'c str>,
}

impl<'c> LoginPage<'c> {
    pub fn new(title: &'c str, flash: Option<&'c str>) -> LoginPage<'c> {
        LoginPage { title, flash }
    }
}

struct UserCookie(usize);

impl<'a, 'r> FromRequest<'a, 'r> for UserCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<UserCookie, ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserCookie(id))
            .or_forward(())
    }
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let title = page_title("Login");

    let context: LoginPage;

    if let Some(ref msg) = flash {
        context = LoginPage::new(&title[..], Some(msg.msg()));
    } else {
        context = LoginPage::new(&title[..], None);
    }

    Template::render("login", &context)
}

#[get("/login")]
fn login_user(_user: UserCookie) -> Redirect {
    Redirect::to("/dashboard")
}

#[post("/login", data = "<login>")]
fn login_submit<'r>(mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    use diesel::prelude::*;
    use schema::users::dsl::*;

    let connection = db::establish_connection();

    let error_msg = "Invalid username / password";

    let user = users
        .filter(email.eq(&login.get().email()))
        .first::<User>(&connection)
        .map_err(|_| Flash::error(Redirect::to("/login"), error_msg))?;

    if !verify_password(
        &user.email.to_string(),
        login.get().password(),
        &user.password.to_string(),
    ) {
        return Err(Flash::error(Redirect::to("/login"), error_msg));
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    Ok(Redirect::to("/login"))
}
