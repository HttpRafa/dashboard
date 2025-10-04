use rocket::{Route, routes};

mod dashboard;
mod feedback;
mod issues;
mod requests;
mod settings;

pub fn web_routes() -> Vec<Route> {
    routes![
        dashboard::dashboard,
        requests::requests,
        issues::issues,
        feedback::feedback,
        settings::settings
    ]
}
