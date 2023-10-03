use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};

use crate::{models::users::User, state::AppState, types::internals::ApiResponse};

/// Gets all IGNs by a specified Discord ID.
///
/// # Returns (Status Code)
/// - Status Code `200` if the query to the database is successful, regardless of
///   how many IGNs are returned.
/// - Status Code `500` if something went wrong internally.
pub async fn get_all_igns_by_id(
    Path(discord_id): Path<String>,
    State(state): State<AppState>,
) -> ApiResponse {
    let mut cursor = get_collection(&state)
        .find(doc! { "ign_lower": discord_id.to_lowercase() }, None)
        .await?;

    let mut all_names = vec![];
    while let Some(user) = cursor.try_next().await? {
        all_names.push(user.ign);
    }

    Ok((StatusCode::OK, Json(all_names)).into_response())
}

fn get_collection(state: &AppState) -> Collection<User> {
    state
        .mongo_client
        .database(&state.env.db_name)
        .collection(&state.env.collections.user_id_name)
}
