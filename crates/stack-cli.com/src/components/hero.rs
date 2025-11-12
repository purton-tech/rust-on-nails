use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            h1 {
                class: "text-6xl max-w-3xl mx-auto text-center pb-6 font-semibold capitalize",
                "Turn Kubernetes into a developer platform"
            }
            h2 {
                class: "subtitle max-w-3xl mx-auto text-center text-lg",
                "Stack layers ingress, identity, databases, and tooling so your team can deploy apps without wrestling with raw cluster YAML."
            }
        }
    }
}
