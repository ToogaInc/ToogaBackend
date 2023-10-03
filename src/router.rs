use axum::Router;

use crate::state::AppState;

/// Creates a router that can be consumed by `axum`.
///
/// # Parameters
/// - `state`: The app state.
///
/// # Returns
/// The router.
pub fn create_router(state: AppState) -> Router {
    Router::new().with_state(state)
}
