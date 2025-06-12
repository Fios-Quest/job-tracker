use crate::{CompanyDetails, Navbar, Route};
use dioxus::prelude::*;
use storage::ApplicationContext;

#[component]
pub fn Details() -> Element {
    let context = use_context::<Signal<ApplicationContext>>();
    let company = context().get_company();
    let role = context().get_role();

    let Some(company) = company else {
        return rsx! {
            "Create or choose a company to get started"
        };
    };

    rsx! {
        div {
            class: "flex flex-col",

            Navbar {
                Link { class: "hover:underline", to: Route::Help {}, "Company Details" }
                if role.is_some() {
                    Link { class: "hover:underline", to: Route::Help {}, "Role Details" }
                    Link { class: "hover:underline", to: Route::Help {}, "Interview Details" }
                    Link { class: "hover:underline", to: Route::Help {}, "Questions" }
                }
            }

            section {
                CompanyDetails { company }
            }
        }

    }
}
