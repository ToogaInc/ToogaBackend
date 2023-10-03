pub mod users;

use axum::{routing::get, Router};

use crate::state::AppState;

use self::users::get_all_igns_by_id;

/// Creates a router that can be consumed by `axum`.
///
/// # Parameters
/// - `state`: The app state.
///
/// # Returns
/// The router.
pub fn create_router(state: AppState) -> Router {
    // Users router
    let user_router = Router::new().route("/", get(get_all_igns_by_id));

    Router::new().nest("users", user_router).with_state(state)
}
