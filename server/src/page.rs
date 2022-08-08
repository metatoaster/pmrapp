use crate::sauron;
use crate::sauron::prelude::*;

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

