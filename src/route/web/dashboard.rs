use maud::Markup;
use rocket::get;

use crate::{
    component::{root::dashboard::DashboardComponent, util::base::BaseComponent},
    route::Page,
};

#[get("/")]
pub async fn dashboard() -> Markup {
    BaseComponent::build(
        "Dashboard | Home",
        Page::Dashboard,
        DashboardComponent::build(),
    )
}
