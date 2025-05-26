use dioxus::prelude::*;

const SUPPORT_CSS: Asset = asset!("/assets/support.css");

#[component]
pub fn Support() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SUPPORT_CSS }

        h1 { "Support" }

        p { class: "warning", "Absolutely do not provide monetary support if you are not working!" }

        p {
            "This app is provided free of charge to help people achieve their maximum potential job
             hunting. If you are not currently working, please save your money."
        }

        p {
            "If you are working and would like to support this project, you can either subscribe
                to my "
            a { href: "https://www.patreon.com/fios_quest", "Patreon" }
            " or send a one off thank you via "
            a { href: "https://ko-fi.com/fios_quest", "Ko-fi" }
            "."
        }
    }
}
