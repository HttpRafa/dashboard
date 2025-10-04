use maud::Markup;
use rocket::get;

use crate::{
    component::{root::requests::RequestsComponent, util::base::BaseComponent},
    route::Page,
};

#[get("/requests")]
pub async fn requests() -> Markup {
    BaseComponent::build(
        "Dashboard | Requests",
        Page::Requests,
        RequestsComponent::build(),
    )
}
