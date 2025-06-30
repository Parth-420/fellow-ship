use axum::{Router, routing::post};
use crate::handlers::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/keypair", post(keypair_handler))
        .route("/token/create", post(token_create_handler))
        .route("/token/mint", post(token_mint_handler))
        .route("/message/sign", post(message_sign_handler))
        .route("/message/verify", post(message_verify_handler))
        .route("/send/sol", post(send_sol_handler))
        .route("/send/token", post(send_token_handler))
} 