use maud::{Markup, html};

use crate::{
    component::util::{
        alert::AlertComponent, header::HeaderComponent, html::HtmlComponent,
        sidebar::SidebarComponent,
    },
    database::model::account::Account,
    route::Page,
};

pub struct BaseComponent;

impl BaseComponent {
    pub fn build(title: &str, page: Page, account: &Account, body: Markup) -> Markup {
        HtmlComponent::build(
            title,
            html! {
                div class="flex" {
                    (SidebarComponent::build(page, account))
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
