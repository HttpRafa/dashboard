use crate::{
    component::util::sidebar::icon::IconComponent,
    database::model::account::Account,
    route::{
        Page, web::admin::accounts::rocket_uri_macro_accounts,
        web::admin::services::rocket_uri_macro_services,
        web::dashboard::rocket_uri_macro_dashboard, web::feedback::rocket_uri_macro_feedback,
        web::issues::rocket_uri_macro_issues, web::requests::rocket_uri_macro_requests,
        web::settings::rocket_uri_macro_settings,
    },
};
use maud::{Markup, html};
use rocket::uri;

pub mod icon;

pub struct SidebarComponent;

impl SidebarComponent {
    pub fn build(page: Page, account: &Account) -> Markup {
        html! {
            aside class="flex flex-col border-r border-zinc-700 text-zinc-400 w-20 min-h-screen space-y-6" {
                nav class="flex-1 flex flex-col" {
                    (IconComponent::build(matches!(page, Page::Dashboard), uri!(dashboard), "dashboard", "Dashboard"))
                    @if account.admin {
                        (IconComponent::build(matches!(page, Page::Services), uri!(services), "apps", "Services"))
                    }
                    (IconComponent::build(matches!(page, Page::Requests), uri!(requests), "assignment_add", "Requests"))
                    (IconComponent::build(matches!(page, Page::Issues), uri!(issues), "pulse_alert", "Issues"))
                    (IconComponent::build(matches!(page, Page::Feedback), uri!(feedback), "feedback", "Feedback"))
                }
                div class="flex flex-col" {
                    (IconComponent::build(false, uri!(dashboard), "frame_person", "Identity"))
                    @if account.admin {
                        (IconComponent::build(matches!(page, Page::Accounts), uri!(accounts), "manage_accounts", "Accounts"))
                    }
                    (IconComponent::build(matches!(page, Page::Settings), uri!(settings), "settings_account_box", "Settings"))
                }
            }
        }
    }
}
