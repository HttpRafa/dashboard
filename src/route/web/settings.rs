use maud::Markup;
use rocket::get;

use crate::{
    component::{root::settings::SettingsComponent, util::base::BaseComponent},
    route::Page,
};

#[get("/settings")]
pub async fn settings() -> Markup {
    BaseComponent::build(
        "Dashboard | Settings",
        Page::Settings,
        SettingsComponent::build(),
    )
}
