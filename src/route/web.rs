use rocket::{Route, routes};

pub mod dashboard;
pub mod feedback;
pub mod issues;
pub mod requests;
pub mod settings;

pub fn web_routes() -> Vec<Route> {
    routes![
        dashboard::dashboard,
        dashboard::dashboard_redirect,
        requests::requests,
        requests::requests_redirect,
        issues::issues,
        issues::issues_redirect,
        feedback::feedback,
        feedback::feedback_redirect,
        settings::settings,
        settings::settings_redirect
    ]
}
