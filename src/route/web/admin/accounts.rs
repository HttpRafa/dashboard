use maud::Markup;
use rocket::{get, response::Redirect, uri};

use crate::{
    component::{root::admin::accounts::AccountsComponent, util::base::BaseComponent},
    database::model::account::Account,
    route::{Page, api::v1::auth::login::rocket_uri_macro_v1_login},
};

#[get("/admin/accounts")]
pub async fn accounts(account: Account) -> Markup {
    BaseComponent::build(
        "Dashboard | Accounts",
        Page::Accounts,
        &account,
        AccountsComponent::build(),
    )
}

#[get("/admin/accounts", rank = 2)]
pub async fn accounts_redirect() -> Redirect {
    Redirect::to(uri!(v1_login))
}
