mod admin;
mod buzzer;
mod game;
mod names;
mod spectator;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use admin::admin_upgrade;
use anyhow::anyhow;
use argh::{from_env, FromArgs};
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{any, get},
    Router,
};
use buzzer::buzzer_upgrade;
use game::{Game, Update};
use lazy_static::lazy_static;
use rspotify::{prelude::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth};
use serde::Deserialize;
use spectator::spectator_upgrade;
use tokio::{
    net::TcpListener,
    sync::{broadcast::channel, RwLock},
};

#[derive(FromArgs, PartialEq, Debug)]
/// Server for songquiz-ng
struct Args {
    /// what address to bind to
    #[argh(positional)]
    bind: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Args = from_env();
    let spotify = AuthCodeSpotify::new(
        Credentials::from_env().ok_or(anyhow!("could not get credentials from environment"))?,
        OAuth::from_env(scopes!("user-modify-playback-state"))
            .ok_or(anyhow!("could not get OAuth config from environment"))?,
    );
    let (channel, _) = channel::<Update>(128);
    let game = Game {
        phase: Default::default(),
        teams: Default::default(),
        song: Default::default(),
        channel,
        spotify,
    };
    let listener = TcpListener::bind(args.bind).await?;
    let app = Router::new()
        .route("/ws/buzzer", any(buzzer_upgrade))
        .route("/ws/admin", any(admin_upgrade))
        .route("/ws/spectator", any(spectator_upgrade))
        .route("/wasm/client.js", get(wasm_client_js))
        .route("/wasm/client_bg.wasm", get(wasm_client_wasm))
        .route("/style.css", get(wasm_css_wrapper))
        .route("/fonts/{file}", get(fonts))
        .route("/buzzer", get(wasm_html_wrapper))
        .route("/admin", get(wasm_html_wrapper))
        .route("/spectator", get(wasm_html_wrapper))
        .route("/authorize", get(authorize))
        .with_state(Arc::new(RwLock::new(game)));
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

const CLIENT_JS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/client.js"));

async fn wasm_client_js() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/javascript")], CLIENT_JS)
}

const CLIENT_WASM: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/client_bg.wasm"));

async fn wasm_client_wasm() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/wasm")], CLIENT_WASM)
}

const INDEX_HTML: &'static str = include_str!("../index.html");

async fn wasm_html_wrapper() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], INDEX_HTML)
}

const STYLE_CSS: &'static str = include_str!("../style.css");

async fn wasm_css_wrapper() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css")], STYLE_CSS)
}

async fn fonts(Path(file): Path<String>) -> axum::response::Result<impl IntoResponse> {
    lazy_static! {
        static ref FONTS: HashMap<&'static str, &'static [u8]> = {
            let mut m = HashMap::<&'static str, &'static [u8]>::new();
            m.insert(
                "FiraSans-Black.ttf",
                include_bytes!("../fonts/FiraSans-Black.ttf"),
            );
            m.insert(
                "FiraSans-BlackItalic.ttf",
                include_bytes!("../fonts/FiraSans-BlackItalic.ttf"),
            );
            m.insert(
                "FiraSans-Bold.ttf",
                include_bytes!("../fonts/FiraSans-Bold.ttf"),
            );
            m.insert(
                "FiraSans-BoldItalic.ttf",
                include_bytes!("../fonts/FiraSans-BoldItalic.ttf"),
            );
            m.insert(
                "FiraSans-ExtraBold.ttf",
                include_bytes!("../fonts/FiraSans-ExtraBold.ttf"),
            );
            m.insert(
                "FiraSans-ExtraBoldItalic.ttf",
                include_bytes!("../fonts/FiraSans-ExtraBoldItalic.ttf"),
            );
            m.insert(
                "FiraSans-ExtraLight.ttf",
                include_bytes!("../fonts/FiraSans-ExtraLight.ttf"),
            );
            m.insert(
                "FiraSans-ExtraLightItalic.ttf",
                include_bytes!("../fonts/FiraSans-ExtraLightItalic.ttf"),
            );
            m.insert(
                "FiraSans-Regular.ttf",
                include_bytes!("../fonts/FiraSans-Regular.ttf"),
            );
            m.insert(
                "FiraSans-Italic.ttf",
                include_bytes!("../fonts/FiraSans-Italic.ttf"),
            );
            m.insert(
                "FiraSans-Light.ttf",
                include_bytes!("../fonts/FiraSans-Light.ttf"),
            );
            m.insert(
                "FiraSans-LightItalic.ttf",
                include_bytes!("../fonts/FiraSans-LightItalic.ttf"),
            );
            m.insert(
                "FiraSans-Medium.ttf",
                include_bytes!("../fonts/FiraSans-Medium.ttf"),
            );
            m.insert(
                "FiraSans-MediumItalic.ttf",
                include_bytes!("../fonts/FiraSans-MediumItalic.ttf"),
            );
            m.insert(
                "FiraSans-SemiBold.ttf",
                include_bytes!("../fonts/FiraSans-SemiBold.ttf"),
            );
            m.insert(
                "FiraSans-SemiBoldItalic.ttf",
                include_bytes!("../fonts/FiraSans-SemiBoldItalic.ttf"),
            );
            m.insert(
                "FiraSans-Thin.ttf",
                include_bytes!("../fonts/FiraSans-Thin.ttf"),
            );
            m.insert(
                "FiraSans-ThinItalic.ttf",
                include_bytes!("../fonts/FiraSans-ThinItalic.ttf"),
            );
            m.insert(
                "MaterialSymbolsOutlined-VariableFont_FILL,GRAD,opsz,wght.ttf",
                include_bytes!(
                    "../fonts/MaterialSymbolsOutlined-VariableFont_FILL,GRAD,opsz,wght.ttf"
                ),
            );
            m
        };
    };
    if let Some(font) = FONTS.get(&file[..]) {
        Ok(*font)
    } else {
        Err(StatusCode::NOT_FOUND.into())
    }
}

#[derive(Deserialize)]
struct OauthResponseQuery {
    code: Option<String>,
}

async fn authorize(
    State(state): State<Arc<RwLock<Game>>>,
    query: Query<OauthResponseQuery>,
) -> axum::response::Result<impl IntoResponse> {
    let game = state.read().await;
    if let Some(code) = &query.code {
        game.spotify
            .request_token(code)
            .await
            .map_err(|_| "Got invalid spotify auth code")?;
        Ok(Redirect::temporary("/admin"))
    } else {
        Ok(Redirect::temporary(
            &game
                .spotify
                .get_authorize_url(true)
                .map_err(|_| "Could not get authorize URL")?,
        ))
    }
}
