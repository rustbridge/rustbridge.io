use failure::Error;
use form::workshop::Workshop as WorkshopForm;
use model::{invite::Invite, workshop::Workshop};
use rocket::{request::Form, response::Redirect};
use rocket_contrib::Template;
use route::{organizer::UserCookie, page_title};
use std::path::PathBuf;

fn home() -> Template {
    let title = page_title("DashBoard");

    let context = json!({
      "title": title,
      "parent": "board/dashboard",
      "content": "",
    });

    Template::render("board/dashboard_content", &context)
}

fn invites(user_id: usize) -> Template {
    use db;
    use diesel::prelude::*;
    use schema::invites::dsl::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();

    let title = page_title("Invites");
    let items: Vec<(Invite, Workshop)> = invites
        .inner_join(workshops)
        .filter(organizer.eq(user_id as i32))
        .get_results(&connection)
        .unwrap();

    let user_invites: Vec<&Invite> = items.iter().map(|(a, _)| a).collect();

    let context = json!({
      "title": title,
      "parent": "board/dashboard",
      "content": "board/your_invites",
      "items": user_invites,
    });

    Template::render("board/dashboard_content", &context)
}

fn workshops(user_id: usize) -> Template {
    use db;
    use diesel::prelude::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();

    let title = page_title("Your Workshops");

    let items: Vec<Workshop> = workshops
        .filter(organizer.eq(user_id as i32))
        .get_results(&connection)
        .unwrap();

    let context = json!({
      "title": title,
      "parent": "board/dashboard",
      "content": "board/your_workshops",
      "items": items,
    });

    Template::render("board/dashboard_content", &context)
}

fn create_workshop() -> Template {
    let title = page_title("Create Workshop");

    let context = json!({
      "title": title,
      "parent": "board/dashboard",
      "content": "board/post_workshop",
    });

    Template::render("board/dashboard_content", &context)
}

#[get("/dashboard/<page..>")]
pub fn dashboard(user: UserCookie, page: PathBuf) -> Template {
    match &page.to_string_lossy().into_owned()[..] {
        "home" => home(),
        "workshops" => workshops(user.0),
        "invites" => invites(user.0),
        "create-workshop" => create_workshop(),
        _ => home(),
    }
}

#[get("/dashboard/<_page..>", rank = 2)]
pub fn unauthenticated_dashboard(_page: PathBuf) -> Redirect {
    Redirect::to("/login")
}

#[post("/dashboard/workshop", data = "<workshop>")]
pub fn post_workshop(user: UserCookie, workshop: Form<WorkshopForm>) -> Result<Redirect, Error> {
    use model::{workshop::NewWorkshop, Sanitize, Resource, Validate};

    let mut new_workshop = NewWorkshop::from(&workshop);
    new_workshop.organizer = Some(user.0 as i32);
    new_workshop.validate()?;
    new_workshop.sanitize()?;
    new_workshop.save()?;

    Ok(Redirect::to("/dashboard/workshops"))
}
