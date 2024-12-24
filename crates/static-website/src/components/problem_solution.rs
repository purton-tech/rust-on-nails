use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(title: String, subtitle: String, video: String, claim: String) -> Element {
    rsx! {
        section {
            class: "md:flex flex-row gap-8 text-center md:text-left mt-16",
            div {
                class: "flex-1",
                div {
                    h1 {
                        class: "text-2xl md:text-5xl font-semibold capitalize",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                }
            }
            div {
                class: "flex-1 mt-8 md:mt-0",
                iframe {
                    class: "w-full aspect-[16/9]",
                    src: "{video}",
                    title: "YouTube video player",
                    "frameborder": "0",
                    allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                    referrerpolicy: "strict-origin-when-cross-origin",
                    allowfullscreen: true,
                }
            }
        }
    }
}
