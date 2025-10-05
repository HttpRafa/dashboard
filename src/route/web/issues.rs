use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::issues::IssuesComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_login},
};

#[get("/issues")]
pub async fn issues(account: Account) -> Markup {
    BaseComponent::build("Dashboard | Issues", Page::Issues, IssuesComponent::build())
}

#[get("/issues", rank = 2)]
pub async fn issues_redirect() -> Redirect {
    Redirect::to(uri!(login))
}
