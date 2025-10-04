use maud::{Markup, html};

use crate::{component::util::sidebar::icon::IconComponent, route::Page};

pub mod icon;

pub struct SidebarComponent;

impl SidebarComponent {
    pub fn build(page: Page) -> Markup {
        html! {
            aside class="flex flex-col border-r border-zinc-700 text-zinc-400 w-20 min-h-screen space-y-6" {
                nav class="flex-1 flex flex-col" {
                    (IconComponent::build(matches!(page, Page::Dashboard), "/", "dashboard", "Dashboard"))
                    (IconComponent::build(matches!(page, Page::Requests), "/requests/", "assignment_add", "Requests"))
                    (IconComponent::build(matches!(page, Page::Issues), "/issues/", "pulse_alert", "Issues"))
                    (IconComponent::build(matches!(page, Page::Feedback), "/feedback/", "feedback", "Feedback"))
                }
                div class="flex flex-col" {
                    (IconComponent::build(false, "https://auth.httxrafa.dev/", "frame_person", "Identity"))
                    (IconComponent::build(matches!(page, Page::Settings), "/settings/", "settings_account_box", "Settings"))
                }
            }
        }
    }
}
