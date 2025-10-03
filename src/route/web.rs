use rocket::{Route, routes};

pub mod dashboard;
pub mod feedback;
pub mod issues;
pub mod requests;
pub mod settings;

pub fn web_routes() -> Vec<Route> {
    routes![
        dashboard::dashboard,
        requests::requests,
        issues::issues,
        feedback::feedback,
        settings::settings
    ]
}
