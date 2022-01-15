use rspotify::model::Device;
use rspotify::model::TrackId;
use rspotify::prelude::*;
use rspotify::scopes;
use rspotify::AuthCodePkceSpotify;
use rspotify::Config;
use rspotify::Credentials;
use rspotify::OAuth;
use rspotify::DEFAULT_API_PREFIX;
use rspotify::DEFAULT_CACHE_PATH;
use rspotify::DEFAULT_PAGINATION_CHUNKS;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter::once;
use std::path::PathBuf;

#[derive(Debug)]
struct AppConfig {}

#[derive(Debug, Deserialize)]
struct ConfigCredentials {
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    let val = get_value();
    let chosen = get_map(val);
    play(chosen).await;
}

fn load_file(path: &str) -> io::Result<String> {
    return fs::read_to_string(path);
}

fn parse_credentials(toml: &str) -> ConfigCredentials {
    return toml::from_str(toml).unwrap();
}

fn load_credentials() -> ConfigCredentials {
    let file_contents = load_file("secrets.toml").expect("ERROR");
    print!("{}", file_contents);
    return parse_credentials(&file_contents);
}

fn get_value() -> i32 {
    return 1;
}

fn get_map(id: i32) -> &'static str {
    let x: HashMap<i32, &str> = [
        // T-Swift
        (1, "1p80LdxRV74UKvL8gnD7ky"),
        // Kanye
        (2, "4fzsfWzRhPawzqhX8Qt9F3"),
    ]
    .into_iter()
    .collect();
    return x[&id];
}

fn create_spotify_creds(config: ConfigCredentials) -> Credentials {
    return Credentials::new(&config.client_id, &config.client_secret);
}

async fn play(id: &str) {
    let creds_obj = load_credentials();
    let creds = create_spotify_creds(creds_obj);

    let oauth = OAuth {
        redirect_uri: "http://localhost:8888/callback".to_string(),
        scopes: scopes!("app-remote-control user-read-playback-state user-modify-playback-state"),
        ..Default::default()
    };

    let config = Config {
        prefix: String::from(DEFAULT_API_PREFIX),
        cache_path: PathBuf::from(DEFAULT_CACHE_PATH),
        pagination_chunks: DEFAULT_PAGINATION_CHUNKS,
        token_cached: true,
        token_refreshing: true,
    };
    let mut spotify = AuthCodePkceSpotify::with_config(creds.clone(), oauth.clone(), config);
    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let devices = spotify.device().await;

    let device = devices.expect("Error grabbing devices");
    // print!("{:?}", device);
    let device_id = "044a9398b7f6ef3ce8b1860ff9d2f08f2fc481a6";
    print!("{}", id);
    spotify
        .start_uris_playback(
            once(&TrackId::from_id(id).unwrap() as &dyn PlayableId),
            Some(device_id),
            None,
            None,
        )
        .await;
}
