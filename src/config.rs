use std::env;
use std::fmt;

/// Application environment
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Environment {
    Local,
    Development,
    Staging,
    Production,
}

impl Environment {
    /// Parse environment from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "local" => Environment::Local,
            "dev" | "development" => Environment::Development,
            "staging" | "stage" => Environment::Staging,
            "prod" | "production" => Environment::Production,
            _ => {
                tracing::warn!("Unknown environment '{}', defaulting to Local", s);
                Environment::Local
            }
        }
    }

    /// Check if environment is production
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    /// Check if environment is local/development
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Local | Environment::Development)
    }

    /// Get recommended CORS settings for this environment
    pub fn cors_permissive(&self) -> bool {
        !self.is_production()
    }

    /// Get recommended log level for this environment
    pub fn default_log_level(&self) -> &str {
        match self {
            Environment::Local => "debug",
            Environment::Development => "debug",
            Environment::Staging => "info",
            Environment::Production => "info",
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Local => write!(f, "local"),
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Current environment
    pub environment: Environment,
    
    /// Database connection URL
    pub database_url: String,
    
    /// JWT secret for token signing
    pub jwt_secret: String,
    
    /// Server port
    pub port: u16,
    
    /// Server host
    pub host: String,
    
    /// JWT token expiration in hours
    pub jwt_expiration_hours: i64,
    
    /// Maximum WebSocket connections per room
    pub max_ws_connections: usize,
    
    /// Allowed CORS origins (empty = allow all)
    pub cors_origins: Vec<String>,
    
    /// Enable request logging
    pub enable_logging: bool,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, env::VarError> {
        // Get environment
        let env_str = env::var("APP_ENV")
            .or_else(|_| env::var("ENVIRONMENT"))
            .unwrap_or_else(|_| "local".to_string());
        let environment = Environment::from_str(&env_str);

        // Get database URL (required)
        let database_url = env::var("DATABASE_URL")?;

        // Get JWT secret (required)
        let jwt_secret = env::var("JWT_SECRET")?;

        // Validate JWT secret length in production
        if environment.is_production() && jwt_secret.len() < 32 {
            panic!("JWT_SECRET must be at least 32 characters in production!");
        }

        // Get port with environment-specific defaults
        let default_port = match environment {
            Environment::Local => "3000",
            Environment::Development => "3001",
            Environment::Staging => "3002",
            Environment::Production => "8080",
        };
        let port = env::var("PORT")
            .unwrap_or_else(|_| default_port.to_string())
            .parse()
            .unwrap_or_else(|_| default_port.parse().unwrap());

        // Get host
        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        // JWT expiration (default: 24 hours)
        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24);

        // Max WebSocket connections per room
        let max_ws_connections = env::var("MAX_WS_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);

        // CORS origins
        let cors_origins = env::var("CORS_ORIGINS")
            .ok()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        // Enable logging (default: true for non-production)
        let enable_logging = env::var("ENABLE_LOGGING")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(!environment.is_production() || true);

        let config = Config {
            environment,
            database_url,
            jwt_secret,
            port,
            host,
            jwt_expiration_hours,
            max_ws_connections,
            cors_origins,
            enable_logging,
        };

        // Log configuration (without secrets)
        tracing::info!("Configuration loaded:");
        tracing::info!("  Environment: {}", config.environment);
        tracing::info!("  Host: {}", config.host);
        tracing::info!("  Port: {}", config.port);
        tracing::info!("  JWT Expiration: {} hours", config.jwt_expiration_hours);
        tracing::info!("  Max WS Connections: {}", config.max_ws_connections);
        tracing::info!("  CORS Origins: {:?}", config.cors_origins);
        tracing::info!("  Enable Logging: {}", config.enable_logging);

        Ok(config)
    }

    /// Get full server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Check if CORS should allow all origins
    pub fn allow_all_cors(&self) -> bool {
        self.cors_origins.is_empty() && self.environment.cors_permissive()
    }
}
