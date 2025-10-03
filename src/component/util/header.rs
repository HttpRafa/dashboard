use maud::{Markup, html};

pub struct HeaderComponent;

impl HeaderComponent {
    pub fn build() -> Markup {
        html! {
            header class="flex items-center justify-between px-6 py-4 h-16 bg-zinc-800" {
                div class="flex items-center space-x-4" {
                    h1 class="text-3xl font-semibold text-gray-200" {
                        "Homelab Services"
                    }
                    img src="/icons/icon.png" alt="Icon" class="w-10 h-10 object-cover rounded-full hidden sm:block";
                }

                div class="flex items-center space-x-4" {
                    a href="/settings/" class="w-8 h-8 rounded-full overflow-hidden border-2 border-gray-700" {
                        img src="https://avatars.githubusercontent.com/u/60099368" alt="User Avatar" class="w-full h-full object-cover";
                    }
                }
            }
        }
    }
}
