pub mod users;

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::state::AppState;

use self::users::{add_association, get_all_igns_by_id, update_association};

/// Creates a router that can be consumed by `axum`.
///
/// # Parameters
/// - `state`: The app state.
///
/// # Returns
/// The router.
pub fn create_router(state: AppState) -> Router {
    // Users router
    let user_router = Router::new()
        .route("/:discord_id", get(get_all_igns_by_id))
        .route("/add", post(add_association))
        .route("/add", put(add_association))
        .route("/reassociate", put(update_association));

    Router::new().nest("users", user_router).with_state(state)
}
