use maud::Markup;
use rocket::get;

use crate::{
    component::{root::issues::IssuesComponent, util::base::BaseComponent},
    route::Route,
};

#[get("/issues")]
pub fn issues() -> Markup {
    BaseComponent::build(
        "Dashboard | Issues",
        Route::Issues,
        IssuesComponent::build(),
    )
}
