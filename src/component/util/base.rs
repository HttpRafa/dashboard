use maud::{Markup, html};

use crate::{
    component::util::{
        alert::AlertComponent, header::HeaderComponent, html::HtmlComponent,
        sidebar::SidebarComponent,
    },
    route::Route,
};

pub struct BaseComponent;

impl BaseComponent {
    pub fn build(title: &str, route: Route, body: Markup) -> Markup {
        HtmlComponent::build(
            title,
            html! {
                div class="flex" {
                    (SidebarComponent::build(route))
                    div class="flex-1" {
                        (HeaderComponent::build())
                        main class="flex-1 p-6" {
                            (AlertComponent::build())
                            (body)
                        }
                    }
                }
            },
        )
    }
}
