mod config;
mod db;
mod errors;
mod models;
mod routes;
mod services;
mod utils;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use services::{auth_service::AuthService, jwt_service::JwtService, message_service::MessageService};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Unified application state
#[derive(Clone, FromRef)]
pub struct AppState {
    pub jwt_service: Arc<JwtService>,
    pub auth_service: Arc<AuthService>,
    pub message_service: Arc<MessageService>,
    pub db: Arc<DatabaseConnection>,
    pub rooms: Arc<RwLock<HashMap<i32, broadcast::Sender<String>>>>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "chat_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    tracing::info!("🚀 Starting chat backend server in {} mode...", config.environment);

    // Establish database connection
    let db = db::establish_connection(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize services with config
    let jwt_service = Arc::new(JwtService::new(&config.jwt_secret, config.jwt_expiration_hours));
    let auth_service = Arc::new(AuthService::new(db.clone(), jwt_service.as_ref().clone()));
    let message_service = Arc::new(MessageService::new(db.clone()));

    // Create unified application state
    let app_state = AppState {
        jwt_service,
        auth_service,
        message_service,
        db: Arc::new(db),
        rooms: Arc::new(RwLock::new(HashMap::new())),
    };

    // Configure CORS based on environment
    let cors = if config.allow_all_cors() {
        tracing::info!("CORS: Allowing all origins (development mode)");
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        tracing::info!("CORS: Restricted to specific origins: {:?}", config.cors_origins);
        use tower_http::cors::AllowOrigin;
        let origins: Vec<_> = config.cors_origins
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Build application with routes
    let app = Router::new()
        // Health check route
        .route("/health", get(routes::health::health_check))
        // Auth routes
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/login", post(routes::auth::login))
        // Protected routes
        .route("/rooms", get(routes::room::get_rooms))
        .route("/rooms/:room_id/messages", post(routes::room::create_message))
        // WebSocket route
        .route("/ws/:room_id", get(routes::websocket::websocket_handler))
        .with_state(app_state)
        .layer(cors);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

