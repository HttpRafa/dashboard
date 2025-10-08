use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::settings::SettingsComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/settings")]
pub async fn settings(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Settings",
        Page::Settings,
        &account,
        SettingsComponent::build(),
    )
}

#[get("/settings", rank = 2)]
pub async fn settings_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
