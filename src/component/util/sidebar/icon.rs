use maud::{Markup, html};

pub struct IconComponent;

impl IconComponent {
    pub fn build(selected: bool, href: &str, icon: &str, text: &str) -> Markup {
        html! {
            @if selected {
                button class="flex flex-col items-center justify-center h-16 bg-zinc-700 shadow-[inset_-3px_0_0_0_#00bc7d] cursor-pointer transition" {
                    span class="material-symbols-outlined" {
                        (icon)
                    }
                    p class="text-xs" {
                        (text)
                    }
                }
            } @else {
                a href=(href) class="flex flex-col items-center justify-center h-16 hover:bg-zinc-700 transition" {
                    span class="material-symbols-outlined" {
                        (icon)
                    }
                    p class="text-xs" {
                        (text)
                    }
                }
            }
        }
    }
}
