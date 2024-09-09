use crate::data::AvailablityStore;
use crate::model::*;
use axum::{
    body::Body,
    extract::{Path, State},
    http::Response,
    response::{ErrorResponse, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

pub type DynAvailStore = Arc<dyn AvailablityStore + Send + Sync>;

//TODO: Check if team names should be unique
pub fn api_routes(store: DynAvailStore) -> Router {
    //TODO: Authentication
    Router::new()
        .route("/team/create", post(create_team))
        .route("/team/by-name/:name", get(get_team))
        .route(
            "/team/by-id/:id",
            get(get_team_by_id).patch(update_team).delete(delete_team),
        )
        .route("/user/create", post(create_user))
        .route("/user/by-name/:name", get(get_user))
        .route(
            "/user/by-id/:id",
            get(get_user_by_id).patch(update_user).delete(delete_user),
        )
        .route("/player/create", post(create_player))
        .route("/player/by-user-id/:user_id", get(get_player_by_user_id))
        //TODO: What is this endpoint for? It's for adding a player to a team, should maybe be team/add-player
        // .route("/player/to-team", get(root))
        .route(
            "/player/:id",
            get(get_player_by_id)
                .patch(update_player)
                .delete(delete_player),
        )
        .route(
            "/available-blocks/create",
            post(create_available_block),
        )
        .route("/available-blocks/by-player/:id", get(get_available_blocks_by_player))
        .route(
            "/available-blocks/by-id/:id",
            get(get_available_block_by_id)
                .patch(update_available_block)
                .delete(delete_available_block),
        )
        //TODO: Implement Route
        .with_state(store)
}
pub type Result<T, E = ErrorResponse> = core::result::Result<T, E>;

pub enum MyError {
    No,
    Yes,
}
impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        Response::builder().status(500).body(Body::empty()).unwrap()
    }
}
impl From<sqlx::error::Error> for MyError {
    fn from(value: sqlx::error::Error) -> Self {
        MyError::Yes
    }
}

async fn get_team(
    State(store): State<DynAvailStore>,
    Path(data): Path<Team>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(team) = store.get_team_by_name(data.name).await? {
        println!("{:?}", team);
        Ok(Json(team))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
    //TODO: Proper error handling
}

async fn get_team_by_id(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(team) = store.get_team_by_id(id).await? {
        println!("{:?}", team);
        Ok(Json(team))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
}

async fn create_team(
    State(store): State<DynAvailStore>,
    Json(data): Json<Team>,
) -> Result<impl IntoResponse, MyError> {
    let team = store.add_team(data).await?;
    //TODO: Better error handling
    Ok(Json(team))
}

// TODO: Grab id from path parameter?
async fn update_team(
    State(store): State<DynAvailStore>,
    Json(data): Json<IdentifiableTeam>,
) -> Result<impl IntoResponse, MyError> {
    let team = store.update_team(data).await?;
    Ok(Json(team))
}

async fn delete_team(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    let team = store.delete_team(id).await?;
    Ok(Json(team))
}

async fn get_user(
    State(store): State<DynAvailStore>,
    Path(data): Path<User>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(user) = store.get_user_by_name(data.name).await? {
        println!("{:?}", user);
        Ok(Json(user))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
    //TODO: Proper error handling
}

async fn get_user_by_id(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(user) = store.get_user_by_id(id).await? {
        println!("{:?}", user);
        Ok(Json(user))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
}

async fn create_user(
    State(store): State<DynAvailStore>,
    Json(data): Json<User>,
) -> Result<impl IntoResponse, MyError> {
    let user = store.add_user(data).await?;
    //TODO: Better error handling
    Ok(Json(user))
}

async fn update_user(
    State(store): State<DynAvailStore>,
    Json(data): Json<IdentifiableUser>,
) -> Result<impl IntoResponse, MyError> {
    let user = store.update_user(data).await?;
    Ok(Json(user))
}

async fn delete_user(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    let user = store.delete_user(id).await?;
    Ok(Json(user))
}

async fn get_player_by_user_id(
    State(store): State<DynAvailStore>,
    Path(data): Path<Player>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(player) = store.get_player_by_user_id(data.user_id).await? {
        println!("{:?}", player);
        Ok(Json(player))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
}

async fn get_player_by_id(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(player) = store.get_player_by_id(id).await? {
        println!("{:?}", player);
        Ok(Json(player))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
}

async fn create_player(
    State(store): State<DynAvailStore>,
    Json(data): Json<Player>,
) -> Result<impl IntoResponse, MyError> {
    let player = store.add_player(data).await?;
    //TODO: Better error handling
    Ok(Json(player))
}

async fn update_player(
    State(store): State<DynAvailStore>,
    Json(data): Json<IdentifiablePlayer>,
) -> Result<impl IntoResponse, MyError> {
    let player = store.update_player(data).await?;
    Ok(Json(player))
}

async fn delete_player(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    let player = store.delete_player(id).await?;
    Ok(Json(player))
}
//TODO: Check types on all path params
//TODO: Update to return Vec of Blocks for a player
async fn get_available_blocks_by_player(
    State(store): State<DynAvailStore>,
    Path(data): Path<Player>,
) -> Result<impl IntoResponse, MyError> {
    //TODO: Fix this method, might error if len(blocks) = 0
        let blocks = store.get_available_blocks_by_player_id(data.user_id).await?;
        println!("{:?}", blocks[0].inner_block);
        Ok(Json(blocks))
}

async fn get_available_block_by_id(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    if let Some(block) = store.get_player_by_id(id).await? {
        println!("{:?}", block);
        Ok(Json(block))
    } else {
        //TODO: Return ERROR CODE 404
        Err(MyError::Yes)
    }
}

async fn create_available_block(
    State(store): State<DynAvailStore>,
    Json(data): Json<AvailableBlock>,
) -> Result<impl IntoResponse, MyError> {
    let block = store.add_available_block(data).await?;
    //TODO: Better error handling
    Ok(Json(block))
}

async fn update_available_block(
    State(store): State<DynAvailStore>,
    Json(data): Json<IdentifiableAvailableBlock>,
) -> Result<impl IntoResponse, MyError> {
    let block = store.update_available_block(data).await?;
    Ok(Json(block))
}

async fn delete_available_block(
    State(store): State<DynAvailStore>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, MyError> {
    let block = store.delete_available_block(id).await?;
    Ok(Json(block))
}

//TODO: REMOVE
async fn root() -> &'static str {
    "Hello, World!"
}
