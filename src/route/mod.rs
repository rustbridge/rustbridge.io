pub mod dashboard;
pub mod organizer;

use failure::Error;
use failure::ResultExt;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use comrak::{markdown_to_html, ComrakOptions};
use form::invite::Invite as InviteForm;
use model::workshop::WorkshopModel;

use rocket::{
    request::Form, response::{NamedFile, Redirect},
};
use rocket_contrib::Template;

#[get("/static/<asset..>", rank = 1)]
pub fn static_asset(asset: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(asset)).ok()
}

#[get("/")]
pub fn about() -> Result<Template, Error> {
    use model::{workshop::Workshop, Resource};
    let page_content = markdown(content_path("about.md").as_path())?;

    type T<'t> = <Workshop<'t> as Resource>::Model;
    let items: Vec<T> = Workshop::read_all()?;

    let context = json!({
      "title": page_title("About"),
      "parent": "main_page/layout",
      "sidebar": "main_page/workshops",
      "content": page_content,
      "items": items,
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

fn send_email(email: &str, code: &str) {
    println!("Sending email to: {} with code: {}", email, code);
}

fn gen_invite_code() -> String {
    use data_encoding;
    use ring::{
        digest, rand::{SecureRandom, SystemRandom},
    };
    const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

    let mut v = [0u8; CREDENTIAL_LEN];
    let _ = SystemRandom.fill(&mut v);
    data_encoding::HEXUPPER.encode(&v[..])
}

#[post("/apply", data = "<invite_form>")]
pub fn apply(invite_form: Form<InviteForm>) -> Result<Redirect, Error> {
    use model::{invite::Invite, invite_confirmation::InviteConfirmation, Resource};

    let invite = Invite::from(&invite_form);
    let invite_id = invite.create()?;
    let code = gen_invite_code();

    InviteConfirmation {
        code: &code,
        invite_id: invite_id.unwrap(),
    }.create()?;

    send_email(&invite.email.unwrap(), &code);

    Ok(Redirect::to("/"))
}

#[get("/confirm_invite/<code>")]
pub fn confirm_invite(code: String) -> Result<Redirect, Error> {
    use model::{invite::Invite, invite_confirmation::InviteConfirmation, Resource};

    type T<'t> = <InviteConfirmation<'t> as Resource>::Model;
    let codes: Vec<T> = InviteConfirmation::read_all()?
        .into_iter()
        .filter(|i| i.code == code)
        .collect();

    if let Some(confirmation) = codes.first() {
        println!("Code found: {:#?}", confirmation);

        InviteConfirmation::delete(confirmation.id as usize)?;

        Invite {
            workshop_id: None,
            email: None,
            attending: Some(true),
            pending: Some(false),
        }.update(confirmation.invite_id as usize)?;

        bail!("Invite Confirmation Success Page Not Implemented")
    }

    bail!("Invite Not Found Page Not Implemented")
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
