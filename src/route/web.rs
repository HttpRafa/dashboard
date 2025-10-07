use rocket::{Route, routes};

use crate::route::web::admin::{accounts, services};

pub mod admin;
pub mod dashboard;
pub mod feedback;
pub mod issues;
pub mod requests;
pub mod settings;

pub fn web_routes() -> Vec<Route> {
    routes![
        dashboard::dashboard,
        dashboard::dashboard_redirect,
        services::services,
        services::services_redirect,
        services::new::new,
        services::new::new_redirect,
        requests::requests,
        requests::requests_redirect,
        issues::issues,
        issues::issues_redirect,
        feedback::feedback,
        feedback::feedback_redirect,
        accounts::accounts,
        accounts::accounts_redirect,
        settings::settings,
        settings::settings_redirect
    ]
}
