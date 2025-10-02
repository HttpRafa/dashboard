use maud::{html, Markup};
use rocket::get;

use crate::{component::util::base::BaseComponent, route::Route};

#[get("/feedback")]
pub fn feedback() -> Markup {
    BaseComponent::build("Dashboard | Feedback", Route::Feedback, html! {
        div class="p-4 sm:p-6 lg:p-8 pt-4 sm:pt-4 lg:pt-4" {
            // Header
            header {
                div class="mb-8" {
                    h1 class="text-3xl font-bold text-white" {
                        "Feedback"
                    }
                    p class="text-slate-300 mt-1" {
                        "View your feedback and submit new feedback"
                    }
                }
                div class="flex justify-between items-center mb-4" {
                    a class="bg-emerald-500/65 hover:bg-emerald-700/65 text-white font-semibold py-2 px-6 rounded-lg transition" {
                        "Submit feedback"
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

            // Feedback
            div class="space-y-3"{
                // Table Header
                div class="hidden md:grid grid-cols-12 gap-4 px-4 py-2 text-lg font-bold text-white" {
                    div class="col-span-12 md:col-span-5" {
                        "Topic"
                    }
                    div class="col-span-12 md:col-span-3" {
                        "Scope"
                    }
                    div class="col-span-12 md:col-span-2" {
                        "Status"
                    }
                    div class="col-span-12 md:col-span-2 text-right" {
                        "Date"
                    }
                }

                // Feedback 1
                div class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition" {
                    // Info
                    div class="col-span-12 md:col-span-5 flex items-center space-x-4" {
                        div class="bg-zinc-800 w-12 h-12 flex items-center justify-center rounded-full" {
                            span class="text-green-400 material-symbols-outlined" {
                                "sports_esports"
                            }
                        }
                        div {
                            p class="font-semibold text-white" {
                                "Add \"Vault Hunter\" to Game Servers (Valheim) List"
                            }
                        }
                    }
                    // Scope
                    div class="col-span-12 md:col-span-3 flex items-center space-x-3" {
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
                    div class="col-span-12 md:col-span-2" {
                        span class="inline-flex items-center bg-green-500/10 text-green-400 text-xs font-semibold px-3 py-1 rounded-full" {
                            span class="w-2 h-2 bg-green-400 rounded-full mr-2 animate-pulse" {}
                            "Acknowledged"
                        }
                    }
                    // Date
                    div class="col-span-12 md:col-span-2 text-right" {
                        p class="text-sm font-semibold text-white" {
                            "29.09.2025 at 05:00 PM"
                        }
                    }
                }

                // Feedback 2
                div class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition" {
                    // Info
                    div class="col-span-12 md:col-span-5 flex items-center space-x-4" {
                        div class="bg-zinc-800 w-12 h-12 flex items-center justify-center rounded-full" {
                            span class="text-blue-400 material-symbols-outlined" {
                                "photo_library"
                            }
                        }
                        div {
                            p class="font-semibold text-white" {
                                "JPG Additional Storage for Photos"
                            }
                        }
                    }
                    // Scope
                    div class="col-span-12 md:col-span-3 flex items-center space-x-3" {
                        img src="https://placehold.co/32x32/334155/FFFFFF?text=PG" alt="Game Servers" class="w-8 h-8 rounded-md";
                        div {
                            p class="font-semibold text-white" {
                                "Photo Gallery"
                            }
                            p class="text-sm text-slate-400" {
                                "Immich"
                            }
                        }
                    }
                    // Status
                    div class="col-span-12 md:col-span-2" {
                        span class="inline-flex items-center bg-blue-500/10 text-blue-400 text-xs font-semibold px-3 py-1 rounded-full" {
                            span class="w-2 h-2 bg-blue-400 rounded-full mr-2 animate-pulse" {}
                            "Pending"
                        }
                    }
                    // Date
                    div class="col-span-12 md:col-span-2 text-right" {
                        p class="text-sm font-semibold text-white" {
                            "29.09.2025 at 05:00 PM"
                        }
                    }
                }
            }
        }
    })
}
