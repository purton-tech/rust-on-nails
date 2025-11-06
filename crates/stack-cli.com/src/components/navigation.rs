use crate::routes::{docs, marketing};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Section {
    None,
    Home,
    Enterprise,
    Partners,
    Pricing,
    Blog,
    Docs,
    Contact,
}

#[component]
pub fn NavItem(
    link: String,
    name: String,
    section: Section,
    current_section: Section,
    class: Option<String>,
) -> Element {
    let mut added_class = "";
    if section == current_section {
        added_class = "underline";
    }
    let class = if let Some(class) = class {
        class
    } else {
        "".to_string()
    };
    let class = format!("{} {}", class, added_class);
    rsx!(
        li {
            a {
                class: format!("{}", class),
                "hx-boost": "true",
                href: link,
                "{name}"
            }
        }
    )
}

#[component]
pub fn Navigation(mobile_menu: Option<Element>, section: Section) -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-50 backdrop-filter backdrop-blur-lg bg-opacity-30",
            div {
                class: "navbar justify-between",
                div {
                    div { class: "dropdown lg:hidden",
                        div {
                            tabindex: "0",
                            role: "button",
                            class: "btn btn-ghost",
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "stroke": "currentColor",
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                class: "h-5 w-5",
                                path {
                                    "d": "M4 6h16M4 12h8m-8 6h16",
                                    "stroke-linejoin": "round",
                                    "stroke-linecap": "round",
                                    "stroke-width": "2"
                                }
                            }
                        }
                        ul {
                            class: "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52",
                            NavItem {
                                link: docs::Index {}.to_string(),
                                name: "Documentation".to_string(),
                                section: Section::Docs,
                                current_section: section.clone(),
                            }
                            {mobile_menu}
                        }
                    }
                    ul {
                        class: "flex flex-row gap-4",
                        li {
                            a {
                                href: marketing::Index {}.to_string(),
                                span {
                                    class: "pl-3 flex flex-row gap-2",
                                    strong {
                                        "STACK"
                                    }
                                }
                            }
                        }
                        li {
                            a {
                                href: marketing::Index {}.to_string(),
                                "Home"
                            }
                        }
                    }
                }
                div { class: "navbar-center hidden lg:flex",
                    ul { class: "menu menu-horizontal px-1",
                    }
                }
                div { class: "hidden lg:flex",
                    ul { class: "menu menu-horizontal",
                        li {
                            a {
                                href: "https://github.com/purton-tech/rust-on-nails",
                                img { src: "https://img.shields.io/github/stars/purton-tech/rust-on-nails" }
                            }
                        }
                        NavItem {
                            link: docs::Index {}.to_string(),
                            name: "Documentation".to_string(),
                            section: Section::Docs,
                            current_section: section.clone(),
                        }
                    }
                }
            }
        }
    }
}
