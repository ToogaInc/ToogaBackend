use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection};

use crate::{
    models::users::User,
    state::AppState,
    types::{body_types::AddUserBody, internals::ApiResponse, query_types::IgnQuery},
};

/// Gets all IGNs by a specified Discord ID.
///
/// # Endpoint Type
/// GET
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
        .find(doc! { "discordId": discord_id }, None)
        .await?;

    let mut all_names = vec![];
    while let Some(user) = cursor.try_next().await? {
        all_names.push(user.ign);
    }

    Ok((StatusCode::OK, Json(all_names)).into_response())
}

/// Associates a Discord ID with the specified IGN. Regardless of whether
/// the Discord ID exists or not prior to this call, this should not error.
///
/// # Endpoint Type
/// POST
///
/// # Returns (Status Code)
/// - Status Code `201` if the query creates a new association.
/// - Status Code `303` if the exact IGN and Discord ID is in the database.
/// - Status Code `409` if the IGN is associated with a different Discord ID.
/// - Status Code `500` if something went wrong internally.
pub async fn add_association(
    State(state): State<AppState>,
    Json(body): Json<AddUserBody>,
) -> ApiResponse {
    let AddUserBody { ign, discord_id } = body;
    let user = get_collection(&state)
        .find_one(doc! { "ignLower": ign.to_lowercase() }, None)
        .await?;

    if let Some(u) = user {
        if u.discord_id == discord_id {
            Ok(StatusCode::SEE_OTHER.into_response())
        } else {
            Ok(StatusCode::CONFLICT.into_response())
        }
    } else {
        let doc_to_insert = User::new(discord_id, ign);
        get_collection(&state)
            .insert_one(doc_to_insert, None)
            .await?;
        Ok(StatusCode::CREATED.into_response())
    }
}

/// Reassociates a Discord ID with the specified IGN. The IGN must be in the
/// database prior to this call being made.
///
/// In other words, changes the Discord ID associated with the IGN to a different
/// Discord ID.
///
/// # Endpoint Type
/// PUT
///
/// # Returns (Status Code)
/// - Status Code `204` if the IGN was found and reassociation occurs.
/// - Status Code `404` if the IGN is not found.
/// - Status Code `500` if something went wrong internally.
pub async fn update_association(
    State(state): State<AppState>,
    Path(discord_id): Path<String>,
    Query(query): Query<IgnQuery>,
) -> ApiResponse {
    let ign = query.ign;
    let user = get_collection(&state)
        .find_one(doc! { "ignLower": ign.to_lowercase() }, None)
        .await?;

    match user {
        None => Ok(StatusCode::NOT_FOUND.into_response()),
        Some(_) => {
            get_collection(&state)
                .update_one(
                    doc! {
                        "ignLower": ign.to_lowercase()
                    },
                    doc! {
                        "discordId": discord_id
                    },
                    None,
                )
                .await?;

            Ok(StatusCode::NO_CONTENT.into_response())
        }
    }
}

/// Unassociates the specified IGN from the Discord ID.
///
/// # Endpoint Type
/// DELETE
///
/// # Returns (Status Code)
/// - Status Code `204` if the IGN was found, is associated with the specified
///   Discord ID, and is now unassociated.
/// - Status Code `404` if the IGN or Discord ID pair is not found.
/// - Status Code `500` if something went wrong internally.
pub async fn delete_ign_from_discord_id(
    State(state): State<AppState>,
    Path(discord_id): Path<String>,
    Query(query): Query<IgnQuery>,
) -> ApiResponse {
    let ign = query.ign;
    let user = get_collection(&state)
        .find_one(
            doc! { "ignLower": ign.to_lowercase(), "discordId": &discord_id },
            None,
        )
        .await?;

    match user {
        None => Ok(StatusCode::NOT_FOUND.into_response()),
        Some(_) => {
            get_collection(&state)
                .delete_one(
                    doc! {
                        "ignLower": ign.to_lowercase(),
                        "discordId": discord_id
                    },
                    None,
                )
                .await?;

            Ok(StatusCode::NO_CONTENT.into_response())
        }
    }
}

/// Unassociates all IGNs from the Discord ID.
///
/// # Endpoint Type
/// DELETE
///
/// # Returns (Status Code)
/// - Status Code `204` if the call was successful.
/// - Status Code `404` if the IGN or Discord ID pair is not found.
/// - Status Code `500` if something went wrong internally.
pub async fn delete_all_ign_from_discord_id(
    State(state): State<AppState>,
    Path(discord_id): Path<String>,
) -> ApiResponse {
    get_collection(&state)
        .delete_many(
            doc! {
                "discordId": discord_id
            },
            None,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

fn get_collection(state: &AppState) -> Collection<User> {
    state
        .mongo_client
        .database(&state.env.db_name)
        .collection(&state.env.collections.user_id_name)
}
