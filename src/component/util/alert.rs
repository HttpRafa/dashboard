use maud::{Markup, html};

pub struct AlertComponent;

impl AlertComponent {
    pub fn build() -> Markup {
        html! {
            div role="status" aria-live="polite" class="flex justify-center w-full mb-4" {
                div class="inline-flex items-center gap-3 rounded-lg px-4 py-2 shadow-sm bg-blue-500/20 border border-blue-400/30 backdrop-blur-sm ring-1 ring-blue-500/10" {
                    span class="w-5 h-5 text-blue-200 flex-none material-symbols-outlined align-middle" {
                        "cloud_alert"
                    }

                    p class="text-sm leading-tight text-blue-50" {
                        span class="font-semibold uppercase tracking-wide text-blue-100" {
                            "SYSTEM ALERT:"
                        }
                        span class="ml-2 text-blue-50" {
                            "Scheduled maintenace on Gameserver Infrastructure on Saturday at 10:00 AM UTC. Expect brief downtime"
                        }
                    }
                }
            }
        }
    }
}
