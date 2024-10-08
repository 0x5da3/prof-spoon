#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Urls are relative to your Cargo.toml file
//pub const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

pub const ASSET1: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/dev/docs/prof-spoon/1PNG5cfdcad44d303f1e.avif";
pub const ASSET2: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/dev/docs/prof-spoon/2PNGa46786e431bc964c.avif";
pub const ASSET3: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/dev/docs/prof-spoon/3PNG7cb75641233a4727.avif";
pub const ASSET4: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/dev/docs/prof-spoon/4PNG8cccaf384fc420b9.avif";

pub const _ASSET1: manganis::ImageAsset = manganis::mg!(image("./public/static/1.PNG"));
pub const _ASSET2: manganis::ImageAsset = manganis::mg!(image("./public/static/2.PNG"));
pub const _ASSET3: manganis::ImageAsset = manganis::mg!(image("./public/static/3.PNG"));
pub const _ASSET4: manganis::ImageAsset = manganis::mg!(image("./public/static/4.PNG"));

#[component]
fn App() -> Element {
    let v = vec![ASSET1, ASSET2, ASSET3, ASSET4];
    let mut opacity_val = use_signal(|| 0);
    let mut selected = use_signal(|| ASSET1);

    let _reload = 16;
    rsx! {
        div { class: "bg-gradient-to-br from-gray-100 to-gray-300 min-h-screen flex justify-center items-center",
            div { class: "w-full max-w-2xl mx-auto bg-gradient-to-b from-rose-50 from-5% via-white via-50% to-white to-95% rounded-lg shadow-lg p-6 space-y-6",
                h1 { class: "text-3xl text-center bg-gradient-to-r from-gray-900 via-gray-700 to-black py-6 rounded-lg text-gray-200 font-serif tracking-wide",
                    "🥄銀乃 匙"
                }
                div { class: "w-full flex justify-center ",
                    img {
                        class: "bg-white w-96 h-96  object-cover rounded-2xl border-2 border-pink-200 shadow-md transition ease-in-out duration-150 cursor-not-allowed ring-1 ring-slate-900/10",
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
                            class: "bg-blue-500 shadow-blue-500/50 shadow-lg text-white font-bold py-1 px-4 rounded-full hover:bg-blue-700 transition duration-300",
                            href: "https://web.iriam.app/s/user/YVR6XsPvqN?uuid=47b0488e",
                            "IRIAM"
                        }
                    }
                    div {
                        a {
                            class: "bg-black shadow-black shadow-md text-white font-bold py-1 px-4 rounded-full hover:bg-gray-700 transition duration-300",
                            href: "https://x.com/spoo_nE",
                            "X (📻)"
                        }
                    }
                    div {
                        a {
                            class: "bg-black shadow-black shadow-md text-white font-bold py-1 px-4 rounded-full hover:bg-gray-700 transition duration-300",
                            href: "https://x.com/ev_1s",
                            "X (🖊)"
                        }
                    }
                    div {
                        a {
                            class: "bg-indigo-600 shadow-indigo-500/50 shadow-lg text-white font-bold py-1 px-4 rounded-full hover:bg-indigo-800 transition duration-300",
                            href: "https://discord.gg/shwHJwbTG4",
                            "Discord"
                        }
                    }
                }
                br {}
                br {}
                footer { class: "bg-gray-800 text-white py-6 text-center",
                    p { "© 銀乃 匙 All Rights Reserved." }
                }
            }
        }
    }
}
