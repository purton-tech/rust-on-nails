use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            h1 {
                class: "text-6xl max-w-3xl mx-auto text-center pb-6 font-semibold capitalize",
                "Create stunning full stack web applications in Rust"
            }
            h2 {
                class: "subtitle max-w-3xl mx-auto text-center text-lg",
                "Highly performant and secure applications in a language you love "
            }
            div {
                class: "flex flex-row justify-center mt-8 mb-8",
                a {
                    class: "btn btn-neutral md:btn-md rounded-full",
                    href: "/docs",
                    "Get Started"
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
