### TODO

* Will require patched teloxide
* Usa facades for telegram related work
* Game related commands redirects to the correct or random game-server
* Updates database with telegram related stuff (users, groups)

# The Mafia Host Bot

## Setup for development

1. Install [ngrok](https://ngrok.com/docs/getting-started/)
2. Setup `ngrok` [static domain](https://dashboard.ngrok.com/cloud-edge/domains)
3. [Create a bot](https://core.telegram.org/bots/features#creating-a-new-bot) for development
4. Setup environment by copying `.env.example` to `.env`
    1. Update `TELOXIDE_TOKEN` value with your bot token from [@BotFather](https://t.me/BotFather)
    2. Update `WEBHOOK_URL` value with domain created in ngrok (`https://<domain>`)

## Running

1. Run `ngrok` with `PORT` from `.env` and [static domain](https://dashboard.ngrok.com/cloud-edge/domains)

```shell
ngrok http <PORT> --domain=<domain>
```

2. Run the bot

```shell
cargo run
```

To restart the bot just rerun last command, ngrok not requires restarting