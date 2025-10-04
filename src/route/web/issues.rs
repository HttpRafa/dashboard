use maud::Markup;
use rocket::get;

use crate::{
    component::{root::issues::IssuesComponent, util::base::BaseComponent},
    route::Page,
};

#[get("/issues")]
pub async fn issues() -> Markup {
    BaseComponent::build("Dashboard | Issues", Page::Issues, IssuesComponent::build())
}
