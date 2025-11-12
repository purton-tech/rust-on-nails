use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            h1 {
                class: "text-6xl max-w-3xl mx-auto text-center pb-6 font-semibold capitalize",
                "Turn Kubernetes into your own PaaS"
            }
            h2 {
                class: "subtitle max-w-3xl mx-auto text-center text-lg",
                "Stack layers ingress, identity, databases, and tooling so you can deploy applications like a managed platform without surrendering cluster control."
            }
        }
    }
}
