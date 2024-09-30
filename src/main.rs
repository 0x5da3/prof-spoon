#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Urls are relative to your Cargo.toml file
pub const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

// pub const ASSET1: &str = manganis::mg!(file(
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/1.PNG"
// ));
// pub const ASSET2: &str = manganis::mg!(file(
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/2.PNG"
// ));
// pub const ASSET3: &str = manganis::mg!(file(
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/3.PNG"
// ));
// pub const ASSET4: &str = manganis::mg!(file(
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/4.PNG"
// ));

// pub const ASSET1: manganis::ImageAsset = manganis::mg!(image("./public/static/1.PNG").preload());
// pub const ASSET2: manganis::ImageAsset = manganis::mg!(image("./public/static/2.PNG").preload());
// pub const ASSET3: manganis::ImageAsset = manganis::mg!(image("./public/static/3.PNG").preload());
// pub const ASSET4: manganis::ImageAsset = manganis::mg!(image("./public/static/4.PNG").preload());

// pub const ASSET1: &str =
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/1PNG5cfdcad44d303f1e.avif";
// pub const ASSET2: &str =
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/2PNGa46786e431bc964c.avif";
// pub const ASSET3: &str =
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/3PNGe9db826187321857.avif";
// pub const ASSET4: &str =
//     "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/4PNGb07e403040713d48.avif";

// pub const ASSET1: &str = "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/1.PNG";
// pub const ASSET2: &str = "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/2.PNG";
// pub const ASSET3: &str = "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/3.PNG";
// pub const ASSET4: &str = "https://github.com/0x5da3/prof-spoon/blob/main/docs/assets/static/4.PNG";

pub const ASSET1: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/1PNG5cfdcad44d303f1e.avif";
pub const ASSET2: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/2PNGa46786e431bc964c.avif";
pub const ASSET3: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/3PNGe9db826187321857.avif";
pub const ASSET4: &str = "https://raw.githubusercontent.com/0x5da3/prof-spoon/refs/heads/main/docs/prof-spoon/4PNGb07e403040713d48.avif";

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let _reload = 5;

    let v = vec![ASSET1, ASSET2, ASSET3, ASSET4];
    let mut opacity_val = use_signal(|| 0);
    let mut selected = use_signal(|| ASSET1);

    //a { href: "https://web.iriam.app/s/user/YVR6XsPvqN?uuid=47b0488e", "イリアム" }
    rsx! {
        link {
            rel: "stylesheet",
            href: "https://github.com/0x5da3/prof-spoon/blob/main/docs/prof-spoon/tailwindcss7115efdd5e257c6f.css"
        }
        script { "https://cdn.tailwindcss.com" }
        div { class: "bg-gradient-to-br from-gray-100 to-gray-300 min-h-screen flex justify-center",
            div { class: "w-full max-w-2xl mx-auto bg-white rounded-lg shadow-lg p-6 space-y-6",
                h1 { class: "text-3xl text-center bg-gradient-to-r from-gray-900 via-gray-700 to-black py-6 rounded-lg text-gray-200 font-serif tracking-wide",
                    "鉄乃 匙"
                }
                div { class: "max-w-screen-sm bg-white",
                    img {
                        class: "transition-opacity duration-500 ease-in-out opacity-{opacity_val}",
                        src: "{selected}",
                        onload: move |_| {
                            *opacity_val.write() = 100;
                        }
                    }
                }
                script {
                    src: "https://platform.twitter.com/widgets.js",
                    r#async: true
                }
                br {}
                div { class: "flex space-x-4  overflow-clip",
                    for asset in v {
                        img {
                            src: "{asset}",
                            class: "w-32 h-32 border-2 border-blue-200 rounded-full object-cover",
                            onclick: move |_| {
                                *selected.write() = asset.clone();
                            }
                        }
                    }
                }
                br {}
                div { class: "max-w-screen-sm bg-white",
                    a {
                        class: "twitter-timeline",
                        href: "https://twitter.com/spoo_nE?ref_src=twsrc%5Etfw"
                    }
                }
            }
        }
    }
}
