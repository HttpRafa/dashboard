use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::feedback::FeedbackComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/feedback")]
pub async fn feedback(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Feedback",
        Page::Feedback,
        &account,
        FeedbackComponent::build(),
    )
}

#[get("/feedback", rank = 2)]
pub async fn feedback_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
