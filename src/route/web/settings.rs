use maud::Markup;
use rocket::get;

use crate::{
    component::{root::settings::SettingsComponent, util::base::BaseComponent},
    route::Route,
};

#[get("/settings")]
pub fn settings() -> Markup {
    BaseComponent::build(
        "Dashboard | Settings",
        Route::Settings,
        SettingsComponent::build(),
    )
}
