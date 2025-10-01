use rocket::get;
use rocket_dyn_templates::{Template, context};

const ID: &str = "feedback";

#[get("/feedback")]
pub fn feedback() -> Template {
    Template::render(
        "route/feedback",
        context! {
            currently_selected: ID,
        },
    )
}
