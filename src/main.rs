use serde::Deserialize;
use spot_pl_clone::PlaylistCloner;
// use std::io::Write;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub auth_token: String,
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // SETUP
    dotenv::dotenv().expect("Can't load end. Take down program");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // INIT
    let env = envy::from_env::<Config>().unwrap();
    let mut pl_cloner = PlaylistCloner::new(
        "1GlOvQEYqEqDboAUxRkDqu",
        "uayfxg5aa64qsu5i01s9bs51z",
        env.auth_token,
    );
    let ok = pl_cloner.clone_playlist().await.unwrap();

    // RUN
    // let songs = pl_cloner.get_playlist_tracks_ids().await;

    // OUTPUT
    // let json_resp = serde_json::to_string(&songs).unwrap();
    // std::io::stdout().write_all(json_resp.as_bytes()).unwrap();

    Ok(ok)
}
