//! Petite interface web de visualisation : API JSON + une page statique embarquée.

use axum::extract::Query;
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use srm_core::extras::ExtraMode;
use srm_core::game::Game;
use srm_core::odds::DEFAULT_RETURN_RATE;
use srm_core::pricing::{self, DEFAULT_MARGIN};
use srm_core::report::{self, Settings};

const INDEX: &str = include_str!("../../web/index.html");
/// Étoile rouge avec un 4.
const FAVICON: &str = include_str!("../../web/favicon.svg");

pub fn router() -> Router {
    let favicon = || async { ([(header::CONTENT_TYPE, "image/svg+xml")], FAVICON) };
    Router::new()
        .route("/", get(|| async { Html(INDEX) }))
        .route("/favicon.svg", get(favicon))
        // Certains navigateurs demandent /favicon.ico sans lire le <link> : même image.
        .route("/favicon.ico", get(favicon))
        .route("/api/tables", get(tables))
        .route("/api/suggest", get(suggest))
        .route("/api/play", post(play))
        .route("/api/stats", post(stats))
}

pub fn serve(host: &str, port: u16) -> anyhow::Result<()> {
    let addr = format!("{host}:{port}");
    tokio::runtime::Runtime::new()?.block_on(async {
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        println!("Interface web sur http://{addr} (Ctrl+C pour arrêter)");
        axum::serve(listener, router()).await?;
        Ok(())
    })
}

struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response()
    }
}

impl From<srm_core::Error> for ApiError {
    fn from(e: srm_core::Error) -> Self {
        Self(StatusCode::BAD_REQUEST, e.to_string())
    }
}

/// Jeu, prix d'une grille et nombre de combinaisons de compléments jouées par grille.
#[derive(Deserialize)]
struct PriceQuery {
    jeu: Option<Game>,
    prix: Option<f64>,
    combos: Option<usize>,
}

impl PriceQuery {
    fn offers(&self) -> Vec<pricing::Offer> {
        let game = self.jeu.unwrap_or_default();
        pricing::catalog(
            self.prix.unwrap_or(game.default_price()),
            self.combos.unwrap_or(1),
        )
    }
}

async fn tables(Query(q): Query<PriceQuery>) -> impl IntoResponse {
    Json(q.offers())
}

// Pas de `#[serde(flatten)]` : avec les paramètres d'URL, il casse la lecture des nombres.
#[derive(Deserialize)]
struct SuggestQuery {
    mise: f64,
    marge: Option<f64>,
    jeu: Option<Game>,
    prix: Option<f64>,
    combos: Option<usize>,
}

async fn suggest(Query(q): Query<SuggestQuery>) -> impl IntoResponse {
    let price = PriceQuery {
        jeu: q.jeu,
        prix: q.prix,
        combos: q.combos,
    };
    Json(pricing::suggest(
        &price.offers(),
        q.mise,
        q.marge.unwrap_or(DEFAULT_MARGIN),
    ))
}

#[derive(Deserialize)]
struct PlayRequest {
    numbers: Vec<u8>,
    t: u8,
    #[serde(default)]
    game: Game,
    /// Étoiles ou numéros chance ; vide ou absent : non pris en compte.
    #[serde(default)]
    extras: Vec<u8>,
    /// `{"mode": "fixed"}` (défaut) ou `{"mode": "reduced", "t": 1}`.
    extra_mode: Option<ExtraMode>,
    grid_price: Option<f64>,
    return_rate: Option<f64>,
    seed: Option<u64>,
}

impl PlayRequest {
    fn settings(&self) -> Result<Settings, ApiError> {
        let s = Settings {
            game: self.game,
            grid_price: self.grid_price.unwrap_or(self.game.default_price()),
            return_rate: self.return_rate.unwrap_or(DEFAULT_RETURN_RATE),
        };
        if !(s.grid_price > 0.0 && s.grid_price.is_finite())
            || !(0.0..=1.0).contains(&s.return_rate)
        {
            return Err(ApiError(
                StatusCode::BAD_REQUEST,
                "prix de grille ou taux de retour invalide".into(),
            ));
        }
        Ok(s)
    }

    fn build(&self, settings: Settings) -> srm_core::Result<report::Report> {
        report::build(
            &self.numbers,
            self.t,
            &self.extras,
            self.extra_mode.unwrap_or(ExtraMode::Fixed),
            settings,
        )
    }
}

async fn play(Json(req): Json<PlayRequest>) -> Result<impl IntoResponse, ApiError> {
    let settings = req.settings()?;
    Ok(Json(req.build(settings)?))
}

async fn stats(Json(req): Json<PlayRequest>) -> Result<impl IntoResponse, ApiError> {
    let settings = req.settings()?;
    // Calcul exact sur environ 2 millions de tirages : hors de la boucle asynchrone.
    let comparison = tokio::task::spawn_blocking(move || {
        let r = req.build(settings)?;
        report::compare(&r, req.seed.unwrap_or(1))
    })
    .await
    .map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))??;
    Ok(Json(comparison))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;

    use super::*;

    async fn call(req: Request<Body>) -> (StatusCode, Value) {
        let res = router().oneshot(req).await.unwrap();
        let status = res.status();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes)
            .unwrap_or(Value::String(String::from_utf8_lossy(&bytes).into_owned()));
        (status, body)
    }

    fn post_json(uri: &str, body: Value) -> Request<Body> {
        Request::post(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn index_is_served() {
        let (status, body) = call(Request::get("/").body(Body::empty()).unwrap()).await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.as_str().unwrap().contains("<title>"));
        assert!(body.as_str().unwrap().contains(r#"href="/favicon.svg""#));
    }

    #[tokio::test]
    async fn favicon_is_a_red_star_with_a_4() {
        for path in ["/favicon.svg", "/favicon.ico"] {
            let res = router()
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::OK);
            assert_eq!(res.headers()[header::CONTENT_TYPE], "image/svg+xml");
            let bytes = res.into_body().collect().await.unwrap().to_bytes();
            let svg = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(svg.contains("<polygon fill=\"#c0392b\""));
            assert!(svg.contains(">4</text>"));
        }
    }

    #[tokio::test]
    async fn tables_and_suggest() {
        let (status, body) = call(
            Request::get("/api/tables?prix=2")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let offers = body.as_array().unwrap();
        assert!(!offers.is_empty());
        let o = &offers[0];
        assert_eq!(
            o["price"].as_f64().unwrap(),
            o["grids"].as_f64().unwrap() * 2.0
        );

        let (status, body) = call(
            Request::get("/api/suggest?mise=100")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        for o in body.as_array().unwrap() {
            let p = o["price"].as_f64().unwrap();
            assert!((100.0 / 1.15..=115.0).contains(&p));
        }
    }

    #[tokio::test]
    async fn play_returns_report() {
        let numbers: Vec<u8> = (10..22).collect();
        let (status, body) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": numbers, "t": 3 }),
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["numbers"].as_array().unwrap().len(), 12);
        let grid = &body["grids"][0];
        assert_eq!(grid.as_array().unwrap().len(), 5);
    }

    #[tokio::test]
    async fn play_rejects_bad_input() {
        let (status, body) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": [1, 2, 3], "t": 3 }),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("non supportés"));

        let numbers: Vec<u8> = (1..=12).collect();
        let (status, _) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": numbers, "t": 3, "grid_price": -1.0 }),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn stats_returns_both_distributions() {
        let numbers: Vec<u8> = (1..=12).collect();
        let (status, body) = call(post_json(
            "/api/stats",
            serde_json::json!({ "numbers": numbers, "t": 3 }),
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let total = |key: &str| -> u64 {
            body[key]["counts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
                .sum()
        };
        assert_eq!(total("system"), 2_118_760);
        assert_eq!(total("random"), 2_118_760);
    }

    #[tokio::test]
    async fn play_with_fixed_and_reduced_extras() {
        let numbers: Vec<u8> = (1..=12).collect();
        let (status, fixed) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": numbers, "t": 3, "extras": [9, 2] }),
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{fixed}");
        assert_eq!(fixed["tickets"], fixed["grids"].as_array().unwrap().len());
        assert_eq!(fixed["extra_sets"], serde_json::json!([[2, 9]]));

        let (status, reduced) = call(post_json(
            "/api/play",
            serde_json::json!({
                "numbers": numbers, "t": 3, "extras": [1, 4, 7, 9],
                "extra_mode": { "mode": "reduced", "t": 1 }
            }),
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{reduced}");
        let grids = reduced["grids"].as_array().unwrap().len() as u64;
        assert_eq!(reduced["tickets"].as_u64().unwrap(), grids * 2);

        // Mode fixe avec trop d'étoiles : erreur explicite.
        let (status, body) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": numbers, "t": 3, "extras": [1, 4, 7] }),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("mode fixe"));
    }

    #[tokio::test]
    async fn loto_game_uses_49_numbers() {
        let with_50: Vec<u8> = (39..=50).collect();
        let (status, body) = call(post_json(
            "/api/play",
            serde_json::json!({ "numbers": with_50, "t": 3, "game": "loto" }),
        ))
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("49"));

        let numbers: Vec<u8> = (38..=49).collect();
        let (status, body) = call(post_json(
            "/api/stats",
            serde_json::json!({ "numbers": numbers, "t": 3, "game": "loto", "extras": [3] }),
        ))
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let total: u64 = body["system"]["counts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .sum();
        assert_eq!(total, 1_906_884);

        let (status, body) = call(
            Request::get("/api/suggest?mise=100&jeu=loto&combos=2")
                .body(Body::empty())
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        for o in body.as_array().unwrap() {
            assert_eq!(
                o["tickets"].as_u64().unwrap(),
                o["grids"].as_u64().unwrap() * 2
            );
            let expected = o["tickets"].as_f64().unwrap() * 2.2;
            assert!((o["price"].as_f64().unwrap() - expected).abs() < 1e-9);
        }
    }
}
