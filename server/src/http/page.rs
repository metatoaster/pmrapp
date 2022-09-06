use client::sauron;
use client::sauron::prelude::*;
use client::{App, Msg};
use crate::http::page::jss::jss;

pub fn index(app: &App) -> Node<Msg> {
    println!("app: {:#?}", app);
    let serialized_state = serde_json::to_string(&app).unwrap();
    node! {
        <!doctype html>
        <html lang="en">
            <head>
                <title>"Example app"</title>
                <script type="module">
                    {text!("
import init, {{ main }} from '/pkg/client.js';
async function start() {{
  await init();
  let app_state = String.raw`{}`;
  await main(app_state);
}}
start();\
                        ", serialized_state)}
                </script>
                <link rel="stylesheet" type="text/css" href="/style/main.css"></link>
            </head>
            { app.view() }
        </html>
    }
}

pub fn style() -> String {
    jss! {
        "body": {
            font_family: "'Arial', sans-serif",
            margin: "0 auto",
            max_width: "90rem",
        },
        "header": {
            display: "flex",
            background_color: "#333",
            color: "#fff",
        },
        "header a": {
            color: "#fff",
            padding: "0.5em 2em",
            text_decoration: "none",
        },
        "header a.active": {
            background_color: "#c00",
        },
    }
}
