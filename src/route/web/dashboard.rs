use maud::Markup;
use rocket::get;

use crate::{
    component::{root::dashboard::DashboardComponent, util::base::BaseComponent},
    route::Route,
};

#[get("/")]
pub fn dashboard() -> Markup {
    BaseComponent::build(
        "Dashboard | Home",
        Route::Dashboard,
        DashboardComponent::build(),
    )
}
