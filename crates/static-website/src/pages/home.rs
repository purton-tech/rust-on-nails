use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navigation::Section;
use crate::components::problem_solution::ProblemSolution;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn home_page() -> String {
    let page = rsx! {
        Layout {
            title: "Rust on Nails",
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

            Footer {}
        }
    };

    crate::render(page)
}
