use rocket::get;
use rocket_dyn_templates::{Template, context};

const ID: &str = "requests";

#[get("/requests")]
pub fn requests() -> Template {
    Template::render(
        "route/requests",
        context! {
            currently_selected: ID,
        },
    )
}
