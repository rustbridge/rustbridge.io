use failure::Error;
use form::workshop::Workshop as WorkshopForm;
use rocket::{request::Form, response::Redirect};
use rocket_contrib::Template;
use route::{organizer::UserCookie, page_title};
use std::path::PathBuf;

fn home() -> Template {
    let context = json!({
      "title": page_title("DashBoard"),
      "parent": "board/dashboard",
      "content": "",
    });

    Template::render("board/dashboard_content", &context)
}

fn invites(user_id: usize) -> Template {
    use model::{invite::Invite, workshop::Workshop, Resource};

    type T<'t> = <Workshop<'t> as Resource>::Model;
    let items: Vec<T> = Workshop::read_all()
        .unwrap()
        .into_iter()
        .filter(|ws| ws.organizer == (user_id as i32))
        .collect();

    type U<'u> = <Invite<'u> as Resource>::Model;
    let invites: Vec<U> = Invite::read_all().unwrap();
    let mut user_invites = vec![];

    items.iter().for_each(|ws| {
        user_invites.extend(
            invites
                .iter()
                .filter(|i| i.workshop_id == ws.id)
                .collect::<Vec<&U>>(),
        );
    });

    let context = json!({
      "title": page_title("Invites"),
      "parent": "board/dashboard",
      "content": "board/your_invites",
      "items": user_invites,
    });

    Template::render("board/dashboard_content", &context)
}

fn workshops(user_id: usize) -> Template {
    use model::{workshop::Workshop, Resource};

    type T<'t> = <Workshop<'t> as Resource>::Model;
    let items: Vec<T> = Workshop::read_all()
        .unwrap()
        .into_iter()
        .filter(|ws| ws.organizer == (user_id as i32))
        .collect();

    let context = json!({
      "title": page_title("Your Workshops"),
      "parent": "board/dashboard",
      "content": "board/your_workshops",
      "items": items,
    });

    Template::render("board/dashboard_content", &context)
}

fn create_workshop() -> Template {
    let context = json!({
      "title": page_title("Create Workshop"),
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
    use model::{workshop::Workshop, Resource, Sanitize, Validate};

    let mut new_workshop = Workshop::from(&workshop);
    new_workshop.organizer = Some(user.0 as i32);
    new_workshop.create()?;

    Ok(Redirect::to("/dashboard/workshops"))
}

#[post("/dashboard/workshop/<id>", data = "<workshop>")]
pub fn update_workshop(
    user: UserCookie,
    id: i32,
    workshop: Form<WorkshopForm>,
) -> Result<Redirect, Error> {
    use model::{workshop::Workshop, Resource};

    let mut existing_workshop = Workshop::from(&workshop);
    existing_workshop.update(id as usize)?;

    Ok(Redirect::to("/dashboard/workshops"))
}
