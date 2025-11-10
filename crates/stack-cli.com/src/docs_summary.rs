use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Setting Up".to_string(),
            pages: vec![Page {
                date: "",
                title: "Introduction",
                description: "Introducing Nails",
                folder: "docs/",
                markdown: include_str!("../content/docs/index.md"),
                image: None,
                author_image: None,
                author: None,
            }],
        }],
    }
}
