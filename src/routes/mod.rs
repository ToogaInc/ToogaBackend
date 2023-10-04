pub mod users;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::state::AppState;

use self::users::{
    add_association, delete_all_ign_from_discord_id, delete_ign_from_discord_id,
    get_all_igns_by_id, update_association,
};

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
        .route("/:discord_id", put(update_association))
        .route("/:discord_id", delete(delete_ign_from_discord_id))
        .route("/:discord_id", delete(delete_all_ign_from_discord_id))
        .route("/add", post(add_association));

    Router::new().nest("users", user_router).with_state(state)
}
