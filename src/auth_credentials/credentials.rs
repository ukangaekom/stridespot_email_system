use lettre::transport::smtp::{authentication::Credentials, PoolConfig};
use lettre::AsyncSmtpTransport;
use lettre::Tokio1Executor;
use std::sync::{Arc, OnceLock};
use once_cell::sync::Lazy;

// ---------- STATICS ----------
static EMAIL: Lazy<String> = Lazy::new(||{
    std::env::var("EMAIL").expect("EMAIL must be set")
});

static APP_PASSWORD: Lazy<String> = Lazy::new(||{
    std::env::var("APP_PASSWORD").expect("APP_PASSWORD must be set")
});

static PROVIDER: Lazy<String> = Lazy::new(||{
    std::env::var("RELAY_PROVIDER").expect("RELAY PROVIDER must be set")
});

const SMTPTRANSPORT: OnceLock<Arc<AsyncSmtpTransport<Tokio1Executor>>> = OnceLock::new();


pub fn get_email() -> &'static str {
    &EMAIL
}

fn get_key() -> &'static str {
    &APP_PASSWORD
}

fn get_provider() -> & 'static str{
    &PROVIDER
}

fn init_mailer() -> Arc<AsyncSmtpTransport<Tokio1Executor>>{

    let creds = Credentials::new(
        get_email().to_string(),
        get_key().to_string()
    );

    let pool_config = PoolConfig::new().max_size(50);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(get_provider())
            .unwrap().credentials(creds)
            .pool_config(pool_config)
            .build();
    
    Arc::new(mailer)

}


pub fn get_mailer() -> Arc<AsyncSmtpTransport<Tokio1Executor>>{
    SMTPTRANSPORT.get_or_init(init_mailer).clone()
}

