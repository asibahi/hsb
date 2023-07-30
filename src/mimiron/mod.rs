use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use clap::Args;
mod types;

#[derive(Args)]
pub struct MimironArgs {
    #[command(flatten)]
    mode: MimironModes,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct MimironModes {
    /// card text to search for
    #[arg(trailing_var_arg = true)]
    card_name: Option<Vec<String>>, // remmeber to join!

    /// deck code to parse
    #[arg(short, long)]
    deck: Option<String>,

    #[arg(short)]
    token: bool,
}

pub fn run(args: MimironArgs) -> Result<()> {
    let creds = get_creds_from_env()?;
    let access_token = get_access_token(creds)?;

    let mode = args.mode;
    if mode.token {
        println!("{access_token}")
    } else if let Some(card_name) = mode.card_name {
        let res = ureq::get("https://us.api.blizzard.com/hearthstone/cards")
            .query("locale", "en_us")
            .query("textFilter", &card_name.join(" "))
            .query("access_token", &access_token)
            .call()?
            .into_json::<types::CardSearchResponse>()?
            .cards;

        for card in res.into_iter().filter(|c| !c.dup).take(5) {
            println!("{:#}", card);
        }
    } else if let Some(deck_string) = mode.deck {
        let res = ureq::get("https://us.api.blizzard.com/hearthstone/deck")
            .query("locale", "en_us")
            .query("code", &deck_string)
            .query("access_token", &access_token)
            .call()?
            .into_json::<types::Deck>()?;

        println!("{res}");
    }

    Ok(())
}

fn get_access_token(creds: String) -> Result<String, anyhow::Error> {
    let access_token = ureq::post("https://oauth.battle.net/token")
        .set("Authorization", &format!("Basic {}", creds))
        .query("grant_type", "client_credentials")
        .call()?
        .into_json::<types::Authorization>()?
        .access_token;
    Ok(access_token)
}

fn get_creds_from_env() -> Result<String, anyhow::Error> {
    dotenvy::dotenv()?;
    let id = std::env::var("BLIZZARD_CLIENT_ID")?;
    let secret = std::env::var("BLIZZARD_CLIENT_SECRET")?;
    let creds = general_purpose::STANDARD_NO_PAD.encode(format!("{}:{}", id, secret).as_bytes());
    Ok(creds)
}
