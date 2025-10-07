use maud::{Markup, html};

pub struct SettingsComponent;

impl SettingsComponent {
    pub fn build() -> Markup {
        html! {
            div class="p-4 sm:p-6 lg:p-8 pt-4 sm:pt-4 lg:pt-4" {
                // Header
                header class="mb-8" {
                    h1 class="text-3xl font-bold text-white" {
                        "Settings"
                    }
                    p class="text-slate-300 mt-1" {
                        "Manage your profile and notification preferences"
                    }
                }

                div class="space-y-8" {
                    // Profile Settings Card
                    div class="bg-zinc-600/30 p-6 rounded-lg shadow-lg" {
                        h2 class="text-xl font-semibold text-white mb-4 flex items-center" {
                            span class="mr-3 text-slate-300 material-symbols-outlined" {
                                "account_circle"
                            }
                            "User Profile"
                        }
                        div class="flex flex-col sm:flex-row items-start sm:items-center space-y-4 sm:space-y-0 sm:space-x-6" {
                            img src="https://placehold.co/96x96/475569/FFFFFF?text=User" alt="User Profile Image" class="w-24 h-24 rounded-full";
                            div class="flex-grow" {
                                label for="github-profile" class="block text-sm font-medium text-slate-200 mb-1" {
                                    "Github Username"
                                }
                                p class="text-xs text-slate-300 mb-2" {
                                    "Your profile image will be synced from GitHub."
                                }
                                div class="flex items-center space-x-2" {
                                    input type="text" id="github-profile" placeholder="e.g., someone" class="w-full bg-zinc-800/30 rounded-md py-2 px-3 text-white placeholder-slate-300 focus:outline-none focus:ring-2 focus:ring-slate-500";
                                    button class="bg-emerald-400/65 hover:bg-emerald-700/65 text-white font-semibold py-2 px-4 rounded-lg shadow-md transition cursor-pointer whitespace-nowrap" {
                                        "Save"
                                    }
                                }
                            }
                        }
                    }

                    // Notification Settings Card
                    div class="bg-zinc-600/30 p-6 rounded-lg shadow-lg" {
                        h2 class="text-xl font-semibold text-white mb-4 flex items-center" {
                            span class="mr-3 text-slate-300 material-symbols-outlined" {
                                "notifications"
                            }
                            "Notifications"
                        }
                        div class="space-y-4" {
                            // Notification Toggle Item
                            div class="flex justify-between items-center" {
                                div {
                                    h3 class="font-medium text-white" {
                                        "Request Status Updates"
                                    }
                                    p class="text-sm text-slate-300" {
                                        "Notify me when my resource requests are approved or denied."
                                    }
                                }
                                label for="status-toggle" class="cursor-pointer" {
                                    input type="checkbox" id="status-toggle" class="toggle-checkbox hidden" checked;
                                    div class="toggle-label relative w-12 h-6 bg-zinc-800 rounded-full transition" {
                                        div class="toggle-dot absolute w-4 h-4 bg-white rounded-full top-1 left-1 transition" {}
                                    }
                                }
                            }

                            hr class="border-zinc-800";

                            // Notification Toggle Item
                            div class="flex justify-between items-center" {
                                div {
                                    h3 class="font-medium text-white" {
                                        "New Service Announcements"
                                    }
                                    p class="text-sm text-slate-300" {
                                        "Notify me when new services are available for requests."
                                    }
                                }
                                label for="status-toggle" class="cursor-pointer" {
                                    input type="checkbox" id="status-toggle" class="toggle-checkbox hidden" checked;
                                    div class="toggle-label relative w-12 h-6 bg-zinc-800 rounded-full transition" {
                                        div class="toggle-dot absolute w-4 h-4 bg-white rounded-full top-1 left-1 transition" {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
