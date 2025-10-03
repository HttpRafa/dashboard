use maud::Markup;
use rocket::get;

use crate::{
    component::{root::feedback::FeedbackComponent, util::base::BaseComponent},
    route::Route,
};

#[get("/feedback")]
pub fn feedback() -> Markup {
    BaseComponent::build(
        "Dashboard | Feedback",
        Route::Feedback,
        FeedbackComponent::build(),
    )
}
