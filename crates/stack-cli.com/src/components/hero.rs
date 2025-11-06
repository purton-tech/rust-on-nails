use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            h1 {
                class: "text-6xl max-w-3xl mx-auto text-center pb-6 font-semibold capitalize",
                "Install a production-ready Rust platform on Kubernetes"
            }
            h2 {
                class: "subtitle max-w-3xl mx-auto text-center text-lg",
                "Stack bundles the operators, policies, and tooling you need to ship secure Rust services in your own cluster."
            }
            div {
                class: "flex flex-row justify-center mt-8 mb-8",
                a {
                    class: "btn btn-neutral md:btn-md rounded-full",
                    href: "#install-stack",
                    "Install Stack CLI"
                }
            }
            div {
                class: "max-w-3xl mx-auto text-center",
                img {
                    src: "/landing-page/screenshot-full-app.png",
                    loading: "lazy",
                    width: "800"
                }
            }
        }
    }
}
