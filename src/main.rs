use leptos::prelude::*
;
use crate::minesweepish::ms_main;

mod minesweepish;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <canvas
            id = "minesweepish"
            style = "
                width: 1280px;
                height: 720px;
            "
        />
    }
}

fn main() {
    mount_to_body(App);
    ms_main::ms_main();
}