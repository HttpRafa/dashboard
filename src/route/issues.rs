use rocket::get;
use rocket_dyn_templates::{Template, context};

const ID: &str = "issues";

#[get("/issues")]
pub fn issues() -> Template {
    Template::render(
        "route/issues",
        context! {
            currently_selected: ID,
        },
    )
}
