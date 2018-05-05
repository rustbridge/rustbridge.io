use rocket;
use rocket_contrib::Template;
use route;

pub fn start() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                route::static_asset,
                route::about,
                route::learn,
                route::volunteer,
                route::organizer::login_page,
                route::organizer::login_user,
                route::organizer::login_submit,
                route::organizer::logout,
                route::dashboard::dashboard,
                route::dashboard::unauthenticated_dashboard,
                route::dashboard::post_workshop,
            ],
        )
        .launch();
}
