use maud::{Markup, html};

pub struct NewServiceComponent;

impl NewServiceComponent {
    pub fn build() -> Markup {
        html! {
            div class="p-4 sm:p-6 lg:p-8 pt-4 sm:pt-4 lg:pt-4" {
                // Header
                header {
                    div class="mb-8" {
                        h1 class="text-3xl font-bold text-white" {
                            "Creating a new service"
                        }
                        p class="text-slate-300 mt-1" {
                            "Fill the form below and pick create"
                        }
                    }
                }

                // Services
                div class="space-y-3"{

                }
            }
        }
    }
}
