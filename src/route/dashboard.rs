use rocket::get;
use rocket_dyn_templates::{Template, context};

// Index = Dashboard selected
const ID: &str = "dashboard";

#[get("/")]
pub fn dashboard() -> Template {
    Template::render(
        "route/dashboard",
        context! {
            currently_selected: ID,
        },
    )
}
