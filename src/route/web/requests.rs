use maud::Markup;
use rocket::get;

use crate::{
    component::{root::requests::RequestsComponent, util::base::BaseComponent},
    route::Route,
};

#[get("/requests")]
pub fn requests() -> Markup {
    BaseComponent::build(
        "Dashboard | Requests",
        Route::Requests,
        RequestsComponent::build(),
    )
}
