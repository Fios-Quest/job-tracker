use dioxus::prelude::*;
use storage::storable::object::role::Role;

#[component]
pub fn PopulatedRoleDescription(role: Role) -> Element {
    // Turn markdown into markup
    let parser = pulldown_cmark::Parser::new(&role.description);
    let mut role_description_html = String::new();
    pulldown_cmark::html::push_html(&mut role_description_html, parser);

    rsx! {
        div { dangerous_inner_html: role_description_html }
    }
}
