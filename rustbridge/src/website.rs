use route;
use rocket;
use rocket_contrib::Template;

pub fn start() {
  rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![
           route::static_asset,
           route::about,
           route::learn,
           route::volunteer
    ]).launch();
}
