use diaspora_be::configuration::get_configuration;
use diaspora_be::startup::run;
use diaspora_be::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main] //actix specific a wrapper around tokio::main
                   // #[tokio::main] //direct tokio main can still be used
async fn main() -> std::io::Result<()> {
    // Logs Configs
    let subscriber = get_subscriber("diaspora_be".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //DB configs
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");

    //App port settings
    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
