use urlencoding::encode;

use axum::{extract::State, http::StatusCode, response::Html, routing::get, Router, Server};
use tokio::sync::mpsc;

use std::process::Command;

const CLIENT_ID: &str = "26329";
const SCOPE: &str = "no_expiry";
const REDIRECT_URI: &str = "http://localhost:8080";

pub async fn auth() -> Result<(), anyhow::Error> {
    let url = format!(
        "https://stackoverflow.com/oauth/dialog?client_id={}&scope={}&redirect_uri={}",
        encode(CLIENT_ID),
        encode(SCOPE),
        encode(REDIRECT_URI),
    );

    let (tx, mut rx) = mpsc::channel::<()>(1);

    let server = Server::bind(&"127.0.0.1:8080".parse()?)
        .serve(
            Router::new()
                .route("/", get(redirect))
                .with_state(tx)
                .into_make_service(),
        )
        .with_graceful_shutdown(async move {
            rx.recv().await;
        });

    Command::new("xdg-open").arg(url).spawn()?; // Opens url on a default browser

    tokio::spawn(server).await??;

    Ok(())
}

async fn redirect(State(tx): State<mpsc::Sender<()>>) -> Result<Html<&'static str>, StatusCode> {
    // This will work, because the server will only terminate once every pending
    // request is completed
    tx.send(())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(
        "
        <!DOCTYPE html>
        <html lang='en'>
            <head></head>
            <body>
                <h1>Successfully extracted token</h1>
                <p id='token'></p>
                <p>
                    Copy this token and expose it to Samson via the
                    SO_ACCESS_TOKEN environment variable.
                </p>
                <script>
                    const fragment = new URLSearchParams(
                        window.location.hash.substring(1),
                    );
                    let token = fragment.get('access_token');

                    const html = document.getElementById('token');

                    html.innerHTML = 'Token: ' + token;
                </script>
            </body>
        </html>
        ",
    ))
}
