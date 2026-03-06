use leptos::prelude::*;

mod minesweepish;

#[component]
pub fn App() -> impl IntoView {
    let start = std::sync::Once::new();
    view! {
        <button on:click = move |_| {
            start.call_once(|| minesweepish::ms_main::ms_main())
        }
        >
        "START MS"
        </button>
        <div
            style = "
                width: 1280px;
                height: 720px;
            "
        >
        <canvas
            id = "minesweepish"
            style = "
                width: 100%;
                height: 100%;
            "
        />
        </div>
    }
}

fn main() {
    //mount_to_body(App);
    minesweepish::ms_main::ms_main()
}