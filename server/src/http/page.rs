use client::sauron;
use client::sauron::prelude::*;
use client::{App, Msg};

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
                        ",serialized_state)}
                </script>
            </head>
            { app.view() }
        </html>
    }
}

