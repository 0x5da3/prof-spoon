#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Urls are relative to your Cargo.toml file
pub const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

pub const ASSET1: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/1PNG5cfdcad44d303f1e.avif";
pub const ASSET2: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/2PNGa46786e431bc964c.avif";
pub const ASSET3: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/3PNGe9db826187321857.avif";
pub const ASSET4: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/4PNGb07e403040713d48.avif";

#[component]
fn App() -> Element {
    let v = vec![ASSET1, ASSET2, ASSET3, ASSET4];
    let mut opacity_val = use_signal(|| 0);
    let mut selected = use_signal(|| ASSET1);

    let _reload = 23;

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/tailwindcss7115efdd5e257c6f.css"
        }
        script { src: "https://cdn.tailwindcss.com" }
        div { class: "bg-gradient-to-br from-gray-100 to-gray-300 min-h-screen flex justify-center items-center",
            div { class: "w-full max-w-2xl mx-auto bg-white rounded-lg shadow-lg p-6 space-y-6",
                h1 { class: "text-3xl text-center bg-gradient-to-r from-gray-900 via-gray-700 to-black py-6 rounded-lg text-gray-200 font-serif tracking-wide",
                    "鉄乃 匙"
                }
                div { class: "w-full flex justify-center bg-white",
                    img {
                        class: "w-96 h-96  object-cover rounded-2xl border-2 border-pink-200 shadow-md",
                        src: "{selected}",
                        onload: move |_| {
                            *opacity_val.write() = 100;
                        }
                    }
                }
                br {}
                div { class: "flex justify-center space-x-4 overflow-clip",
                    for asset in v {
                        img {
                            src: "{asset}",
                            class: "w-16 h-16 border-2 border-blue-200 rounded-full object-cover",
                            onclick: move |_| {
                                *selected.write() = asset;
                            }
                        }
                    }
                }
                br {}
                div { class: "flex justify-center space-x-4 ",
                    div {
                        a {
                            class: "bg-[#6777FF] text-white font-bold py-1 px-4 rounded-full hover:bg-blue-700 transition duration-300",
                            href: "https://web.iriam.app/s/user/YVR6XsPvqN?uuid=47b0488e",
                            "IRIAM"
                        }
                    }
                    div {
                        a {
                            class: "bg-[#000000] text-white font-bold py-1 px-4 rounded-full hover:bg-gray-700 transition duration-300",
                            href: "https://x.com/spoo_nE",
                            "X"
                        }
                    }
                }
            }
        }
    }
}
