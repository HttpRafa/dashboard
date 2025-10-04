use maud::Markup;
use rocket::get;

use crate::{
    component::{root::feedback::FeedbackComponent, util::base::BaseComponent},
    route::Page,
};

#[get("/feedback")]
pub async fn feedback() -> Markup {
    BaseComponent::build(
        "Dashboard | Feedback",
        Page::Feedback,
        FeedbackComponent::build(),
    )
}
