use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navigation::Section;
use crate::components::problem_solution::ProblemSolution;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn home_page() -> String {
    let page = rsx! {
        Layout {
            title: "Stack",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "p-5 mt-16 mx-auto max-w-5xl",
                Hero {
                }

                ProblemSolution {
                    video: "https://www.youtube.com/embed/Wd8EqeAeeck?si=BETsJN_94VoyQrcI",
                    title: "Server side rendering and a sprinkle of interactivity",
                    subtitle: "SSR gives you a low code simple way to build applications and we gave you several ways to add interactivity when needed.",
                    claim: "and join hundreds of global installations!"
                }
            }


            // Section heading
            div {
                class: "py-8 text-center",
                h1 {
                    class: "text-3xl font-semibold",
                    "All your web application needs, covered"
                }
            }

            // Grid container
            div {
                class: "max-w-7xl mx-auto px-4",

                // First row of cards
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6",

                    // Card 1
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-blue-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-blue-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "SQL into Functions"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "Auto generate Rust functions from SQL definitions"
                            }
                        }
                    }

                    // Card 2
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-purple-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-purple-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "UI Components"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "React like components to buold your UI."
                            }
                        }
                    }

                    // Card 3
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-pink-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-pink-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "Type safe routing"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "Axum for high performance routing"
                            }
                        }
                    }
                }

                // Second row of cards
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6 mt-6",

                    // Card 4
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-yellow-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-yellow-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "CI/CD"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "Find emails"
                            }
                        }
                    }

                    // Card 5
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-green-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-green-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "Kubernetes"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "Autosync tools"
                            }
                        }
                    }

                    // Card 6
                    div {
                        class: "border border-black",
                        div {
                            class: "bg-orange-200 h-32 flex items-center justify-center",
                            svg {
                                class: "w-16 h-16 text-orange-600",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                circle { cx: "12", cy: "12", r: "10" }
                            }
                        }
                        div {
                            class: "bg-black p-4",
                            h2 {
                                class: "text-lg font-bold text-white",
                                "Islands"
                            }
                            p {
                                class: "text-sm text-gray-400",
                                "Scale outreach"
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    };

    crate::render(page)
}
