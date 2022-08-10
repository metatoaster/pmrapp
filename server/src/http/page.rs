use client::sauron;
use client::sauron::prelude::*;

pub fn index() -> Node<()> {
    node! {
        <!doctype html>
        <html lang="en">
            <head>
               <title>"Example app"</title>
            </head>
        </html>
    }
}

