use maud::{Markup, html};

use crate::{component::util::sidebar::icon::IconComponent, route::Route};

pub mod icon;

pub struct SidebarComponent;

impl SidebarComponent {
    pub fn build(route: Route) -> Markup {
        html! {
            aside class="flex flex-col border-r border-zinc-700 text-zinc-400 w-20 min-h-screen space-y-6" {
                nav class="flex-1 flex flex-col" {
                    (IconComponent::build(matches!(route, Route::Dashboard), "/", "dashboard", "Dashboard"))
                    (IconComponent::build(matches!(route, Route::Requests), "/requests/", "assignment_add", "Requests"))
                    (IconComponent::build(matches!(route, Route::Issues), "/issues/", "pulse_alert", "Issues"))
                    (IconComponent::build(matches!(route, Route::Feedback), "/feedback/", "feedback", "Feedback"))
                }
                div class="flex flex-col" {
                    (IconComponent::build(false, "https://auth.httxrafa.dev/", "frame_person", "Identity"))
                    (IconComponent::build(matches!(route, Route::Settings), "/settings/", "settings_account_box", "Settings"))
                }
            }
        }
    }
}
