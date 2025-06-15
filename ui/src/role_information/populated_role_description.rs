use dioxus::prelude::*;
use std::sync::Arc;
use storage::prelude::*;

#[component]
pub fn PopulatedRoleDescription(role: Arc<Role>) -> Element {
    // Turn markdown into markup
    let parser = pulldown_cmark::Parser::new(&role.description);
    let mut role_description_html = String::new();
    pulldown_cmark::html::push_html(&mut role_description_html, parser);

    rsx! {
        div { dangerous_inner_html: role_description_html }
    }
}
