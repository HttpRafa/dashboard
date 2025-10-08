use maud::{Markup, html};
use rocket::uri;

use crate::route::api::v1::services::rocket_uri_macro_v1_services;

pub struct CreateServiceComponent;

impl CreateServiceComponent {
    pub fn build() -> Markup {
        html! {
            div class="p-4 sm:p-6 lg:p-8 pt-4 sm:pt-4 lg:pt-4" {
                // Header
                header {
                    div class="mb-8" {
                        h1 class="text-3xl font-bold text-white" {
                            "Create New Service"
                        }
                        p class="text-slate-300 mt-1" {
                            "Fill out the details below to add a new service to the dashboard."
                        }
                    }
                }

                // Form
                div class="bg-zinc-600/30 p-6 rounded-lg shadow-lg" {
                    h2 class="text-xl font-semibold text-white mb-6 flex items-center" {
                        span class="mr-3 text-slate-400 material-symbols-outlined" {
                            "add_circle"
                        }
                        "Service Details"
                    }

                    form class="space-y-6" action=(uri!(v1_services)) method="POST" {
                        // Row 1
                        div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                            // Name
                            div {
                                label for="name" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Service Name"
                                }
                                input type="text" id="name" name="name" placeholder="e.g., Password Manager" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                            }

                            // Technology
                            div {
                                label for="technology" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Technology"
                                }
                                input type="text" id="technology" name="technology" placeholder="e.g., Vaultwarden" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                            }
                        }

                        // Row 2
                        div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                            // Website
                            div {
                                label for="website" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Github Repo / Website"
                                }
                                input type="url" id="website" name="website" placeholder="https://github.com/user/repo" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                            }

                            // Instance
                            div {
                                label for="instance" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Instance"
                                }
                                input type="url" id="instance" name="instance" placeholder="https://your-service-url.com" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                            }
                        }

                        // Row 3
                        div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                            // Icon
                            div {
                                label for="icon" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Simple Icon Name"
                                }
                                input type="text" id="icon" name="icon" placeholder="e.g., github, vaultwarden" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                                p class="text-xs text-slate-300 mt-1" {
                                    "Find icons at "
                                    a href="https://simpleicons.org/" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline" {
                                        "simpleicons.org"
                                    }
                                }
                            }

                            // Display Text
                            div {
                                label for="text" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Link Display Text"
                                }
                                input type="text" id="text" name="text" placeholder="e.g., Manage Passwords" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                            }
                        }

                        // Row 4
                        div {
                            label class="block text-sm font-medium text-slate-200 mb-2" {
                                "Icon Size"
                            }
                            div class="grid grid-cols-1 md:grid-cols-2 gap-6" {
                                div {
                                    label for="icon_width" class="block text-xs font-medium text-slate-300 mb-1" {
                                        "Width"
                                    }
                                    input type="number" id="icon_width" name="icon_width" placeholder="8" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                                    p class="text-xs text-slate-300 mt-1" {
                                        "Generates Tailwind 'w-x' class."
                                    }
                                }
                                div {
                                    label for="icon_height" class="block text-xs font-medium text-slate-300 mb-1" {
                                        "Height"
                                    }
                                    input type="number" id="icon_height" name="icon_height" placeholder="8" class="w-full bg-zinc-800/75 rounded-md py-2 px-3 text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-slate-500";
                                    p class="text-xs text-slate-300 mt-1" {
                                        "Generates Tailwind 'h-x' class."
                                    }
                                }
                            }
                        }

                        // Actions
                        div class="pt-4 flex justify-end" {
                            button type="submit" class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded-lg shadow-md transition" {
                                "Create Service"
                            }
                        }
                    }
                }
            }
        }
    }
}
