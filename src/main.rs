use crate::todo::App;
use leptos::*;

pub mod todo;

fn main() {
    mount_to_body(|cx| view! {cx, <App />});
}
