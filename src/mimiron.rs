use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use clap::Args;

mod types;

const CREDS: &str = "Credentials.toml";

#[derive(Args)]
pub struct MimironArgs {
    /// text to search for
    #[arg(trailing_var_arg = true)]
    search_term: Vec<String>, // remmeber to join!
}

pub fn run(args: MimironArgs) -> Result<()> {
    let creds = get_creds_from_file()?;

    let access_token = ureq::post("https://oauth.battle.net/token")
        .set("Authorization", &format!("Basic {}", creds))
        .query("grant_type", "client_credentials")
        .call()?
        .into_json::<types::Authorization>()?
        .access_token;

    let res = ureq::get("https://us.api.blizzard.com/hearthstone/cards")
        .query("locale", "en_us")
        .query("textFilter", &args.search_term.join(" "))
        .query("access_token", &access_token)
        .call()?
        .into_json::<types::Response>()?
        .cards;

    for card in res.into_iter().filter(|c| !c.dup).take(5) {
        println!("{}", card);
    }

    Ok(())
}

fn get_creds_from_file() -> Result<String, anyhow::Error> {
    let creds = std::fs::read_to_string(CREDS)?;
    let creds: types::Credentials = toml::from_str(&creds)?;
    let creds = general_purpose::STANDARD_NO_PAD.encode(
        format!(
            "{}:{}",
            creds.blizzard_client_id, creds.blizzard_client_secret
        )
        .as_bytes(),
    );
    Ok(creds)
}