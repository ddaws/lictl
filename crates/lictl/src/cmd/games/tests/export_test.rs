use crate::{
    cmd::games::{Format, export},
    context::Context,
};
use anyhow::Result;
use mockito::{mock, server_url};
use serde_json::{Value, json};

#[tokio::test]
async fn test_export_pgn_format() -> Result<()> {
    let pgn_content = r#"[Event "Rated Blitz game"]
[Site "https://lichess.org/abcd1234"]
[Date "2025.05.01"]
[Round "-"]
[White "Player1"]
[Black "Player2"]
[Result "1-0"]
[UTCDate "2025.05.01"]
[UTCTime "12:34:56"]
[WhiteElo "1500"]
[BlackElo "1450"]
[WhiteRatingDiff "+8"]
[BlackRatingDiff "-8"]
[Variant "Standard"]
[TimeControl "180+0"]
[ECO "C00"]
[Opening "French Defense"]
[Termination "Normal"]

1. e4 e6 2. d4 d5 3. exd5 exd5 4. Nf3 Nf6 5. Bd3 Bd6 1-0"#;

    let _m = mock("GET", "/api/game/export/abcd1234")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(pgn_content)
        .create();

    let ctx = Context::new_with_base_url(&server_url())?;
    let result = export::run(&ctx, "abcd1234", Format::Pgn).await?;

    assert!(matches!(result, Value::String(_)));
    if let Value::String(pgn) = result {
        assert_eq!(pgn, pgn_content);
    }

    Ok(())
}

#[tokio::test]
async fn test_export_json_format() -> Result<()> {
    let json_content = json!({
        "id": "abcd1234",
        "rated": true,
        "variant": "standard",
        "speed": "blitz",
        "perf": "blitz",
        "createdAt": 1714636496000,
        "lastMoveAt": 1714636796000,
        "status": "mate",
        "players": {
            "white": {
                "user": {
                    "name": "Player1",
                    "id": "player1"
                },
                "rating": 1500,
                "ratingDiff": 8
            },
            "black": {
                "user": {
                    "name": "Player2",
                    "id": "player2"
                },
                "rating": 1450,
                "ratingDiff": -8
            }
        },
        "winner": "white",
        "pgn": "[Event \"Rated Blitz game\"]\n[Site \"https://lichess.org/abcd1234\"]\n[Date \"2025.05.01\"]\n[Round \"-\"]\n[White \"Player1\"]\n[Black \"Player2\"]\n[Result \"1-0\"]\n[UTCDate \"2025.05.01\"]\n[UTCTime \"12:34:56\"]\n[WhiteElo \"1500\"]\n[BlackElo \"1450\"]\n[WhiteRatingDiff \"+8\"]\n[BlackRatingDiff \"-8\"]\n[Variant \"Standard\"]\n[TimeControl \"180+0\"]\n[ECO \"C00\"]\n[Opening \"French Defense\"]\n[Termination \"Normal\"]\n\n1. e4 e6 2. d4 d5 3. exd5 exd5 4. Nf3 Nf6 5. Bd3 Bd6 1-0"
    });

    let _m = mock("GET", "/api/game/export/abcd1234?pgnInJson=true")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json_content.to_string())
        .create();

    let ctx = Context::new_with_base_url(&server_url())?;
    let result = export::run(&ctx, "abcd1234", Format::Json).await?;

    assert!(matches!(result, Value::Object(_)));
    assert_eq!(result["id"], "abcd1234");
    assert!(result["pgn"].is_string());
    assert_eq!(result["winner"], "white");

    Ok(())
}
