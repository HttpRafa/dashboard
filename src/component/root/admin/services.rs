use maud::{Markup, html};
use rocket::uri;

use crate::route::web::admin::services::new::rocket_uri_macro_new;

pub mod new;

pub struct ServicesComponent;

impl ServicesComponent {
    pub fn build() -> Markup {
        html! {
            div class="p-4 sm:p-6 lg:p-8 pt-4 sm:pt-4 lg:pt-4" {
                // Header
                header {
                    div class="mb-8" {
                        h1 class="text-3xl font-bold text-white" {
                            "Services"
                        }
                        p class="text-slate-300 mt-1" {
                            "Manage the services that are displayed on the dashboard"
                        }
                    }
                    div class="flex justify-between items-center mb-4" {
                        a href=(uri!(new)) class="bg-blue-500/65 hover:bg-blue-700/65 text-white font-semibold py-2 px-6 rounded-lg transition" {
                            "New service"
                        }
                        button class="flex items-center bg-zinc-600/30 hover:bg-zinc-600/70 text-white font-semibold py-2 px-4 rounded-lg transition cursor-pointer" {
                            span {
                                "Filter"
                            }
                            span class="text-lg ml-1 material-symbols-outlined" {
                                "expand_more"
                            }
                        }
                    }
                }

                // Services
                div class="space-y-3"{
                    // Table Header
                    div class="hidden md:grid grid-cols-12 gap-4 px-4 py-2 text-lg font-bold text-white" {
                        div class="col-span-12 md:col-span-4" {
                            "Service"
                        }
                        div class="col-span-12 md:col-span-3" {
                            "Users"
                        }
                        div class="col-span-12 md:col-span-3" {
                            "Date"
                        }
                        div class="col-span-12 md:col-span-2 text-right" {
                            "Actions"
                        }
                    }

                    // Service 1
                    div class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition" {
                        // Service
                        div class="col-span-12 md:col-span-4 flex items-center space-x-3" {
                            img src="https://placehold.co/32x32/334155/FFFFFF?text=GS" alt="Game Servers" class="w-8 h-8 rounded-md";
                            div {
                                p class="font-semibold text-white" {
                                    "Game Servers"
                                }
                                p class="text-sm text-slate-400" {
                                    "Pelican"
                                }
                            }
                        }
                        // Status
                        div class="col-span-12 md:col-span-3" {
                            span class="inline-flex items-center bg-green-500/10 text-green-400 text-xs font-semibold px-3 py-1 rounded-full" {
                                "2"
                            }
                            span class="inline-flex items-center text-gray-400 text-xs font-semibold px-3 py-1 rounded-full" {
                                "/"
                            }
                            span class="inline-flex items-center bg-blue-500/10 text-blue-400 text-xs font-semibold px-3 py-1 rounded-full" {
                                "5"
                            }
                        }
                        // Date
                        div class="col-span-12 md:col-span-3" {
                            p class="text-sm font-semibold text-white" {
                                "29.09.2025 at 05:00 PM"
                            }
                        }
                        // Actions
                        div class="col-span-12 md:col-span-2 text-right" {
                            p class="text-sm font-semibold text-white" {
                                "29.09.2025 at 05:00 PM"
                            }
                        }
                    }
                }
            }
        }
    }
}
