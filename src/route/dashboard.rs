use db;
use form::workshop::Workshop;
use model::{invite::Invite as InviteModel, workshop::Workshop as WorkshopModel};
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
    use diesel::prelude::*;
    use schema::invites::dsl::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();

    let title = page_title("Invites");
    let items: Vec<InviteModel> = invites
      .inner_join(workshops)
      .filter(organizer.eq(user_id as i32))
      .get_results(&connection)
      .unwrap()
      .into_iter()
      .map(|(invite, _): (InviteModel, WorkshopModel)| invite)
      .collect();

    let context = json!({
      "title": title,
      "parent": "board/dashboard",
      "content": "",
    });

    Template::render("board/dashboard_content", &context)
}

fn workshops(user_id: usize) -> Template {
    use diesel::prelude::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();

    let title = page_title("Your Workshops");
    let items: Vec<WorkshopModel> = workshops
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
pub fn post_workshop(user: UserCookie, workshop: Form<Workshop>) -> Option<Redirect> {
    use diesel::prelude::*;
    use schema::workshops::dsl::*;

    let connection = db::establish_connection();

    let new_workshop = (
        name.eq(workshop.get().name()),
        organizer.eq(user.0 as i32),
        description.eq(workshop.get().description()),
        location.eq(workshop.get().location()),
        event_date.eq(workshop.get().date()),
        start_time.eq(workshop.get().start_time()),
        end_time.eq(workshop.get().end_time()),
        private.eq(workshop.get().private()),
    );

    let _ = ::diesel::insert_into(workshops)
        .values(&new_workshop)
        .execute(&connection);

    Some(Redirect::to("/dashboard/workshops"))
}
