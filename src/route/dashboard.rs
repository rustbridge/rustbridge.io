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
                .collect::<Vec<&U>>()
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

#[get("/dashboard/update-workshop/<id>")]
fn update_workshop(user: UserCookie, id: usize) -> Result<Template, Error> {
  use model::{workshop::Workshop, Resource};

  type T<'t> = <Workshop<'t> as Resource>::Model;
  let items: Vec<T> = Workshop::read_all()
    .unwrap()
    .into_iter()
    .filter(|x| (user.0 as i32) == x.organizer)
    .filter(|x| x.id == (id as i32))
    .collect();

  if !items.is_empty() {
    let item = items.first().unwrap();

    let context = json!({
      "title": page_title("Update Workshop"),
      "parent": "board/dashboard",
      "content": "board/update_workshop",
      "id": id,
      "current_name": item.name,
      "current_desc": item.description,
      "current_loc": item.location,
      "current_date": item.date.date(),
      "current_start_time": item.start_time.time(),
      "current_end_time": item.end_time.time(),
      "current_private": item.private,
    });

    println!("{:#?}", context);

    Ok(Template::render("board/dashboard_content", &context))

  } else {
    bail!("page not found")
  }
}

#[get("/dashboard/<page..>", rank = 2)]
pub fn dashboard(user: UserCookie, page: PathBuf) -> Template {
    match &page.to_string_lossy().into_owned()[..] {
        "home" => home(),
        "workshops" => workshops(user.0),
        "invites" => invites(user.0),
        "create-workshop" => create_workshop(),
        _ => home(),
    }
}

#[get("/dashboard/<_page..>", rank = 3)]
pub fn unauthenticated_dashboard(_page: PathBuf) -> Redirect {
    Redirect::to("/login")
}

#[post("/dashboard/workshop", data = "<workshop>")]
pub fn post_workshop(user: UserCookie, workshop: Form<WorkshopForm>) -> Result<Redirect, Error> {
    use model::{workshop::Workshop, Resource};

    let mut new_workshop = Workshop::from(&workshop);
    new_workshop.organizer = Some(user.0 as i32);
    new_workshop.create()?;

    Ok(Redirect::to("/dashboard/workshops"))
}

#[post("/dashboard/update-workshop", data = "<workshop>")]
pub fn put_workshop(
    user: UserCookie,
    workshop: Form<WorkshopForm>,
) -> Result<Redirect, Error> {
    use model::{workshop::Workshop, Resource};

    let mut existing_workshop = Workshop::from(&workshop);
    existing_workshop.update(workshop.get().model_id().unwrap())?;

    Ok(Redirect::to("/dashboard/workshops"))
}
