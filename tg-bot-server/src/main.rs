use dotenv::dotenv;
use teloxide::{prelude::*, update_listeners::webhooks};

const ENV_PORT: &str = "PORT";
const WEBHOOK_URL: &str = "WEBHOOK_URL";

fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {env} env variable"))
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    pretty_env_logger::init();

    log::info!("Starting ngrok ping-pong bot...");

    let bot = Bot::from_env();

    let port = get_env(ENV_PORT);
    let url = get_env(WEBHOOK_URL).parse().unwrap();

    let addr = format!("0.0.0.0:{port}").parse().unwrap();
    let mut options = webhooks::Options::new(addr, url);

    async fn root() -> &'static str {
        "Hello, World!"
    }

    options.drop_pending_updates = true;
    options.extra_routes.push(("/extra", axum::routing::get(root)));

    let listener = webhooks::axum(bot.clone(), options)
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
        .await;
}