use rocket::get;
use rocket_dyn_templates::{Template, context};

const ID: &str = "settings";

#[get("/settings")]
pub fn settings() -> Template {
    Template::render(
        "route/settings",
        context! {
            currently_selected: ID,
        },
    )
}
