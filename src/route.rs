pub mod dashboard;
pub mod feedback;
pub mod issues;
pub mod requests;
pub mod settings;

pub enum Route {
    Dashboard,
    Feedback,
    Issues,
    Requests,
    Settings,
}