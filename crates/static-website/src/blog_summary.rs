use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2024-12-24",
                    title: "Rust on Nails : Version 0.1",
                    description: "Rust on Nails : Version 0.1",
                    folder: "blog/version-0.1/",
                    markdown: include_str!("../content/blog/version-0.1/index.md"),
                    image: Some("/blog/version-0.1/example-app.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
            ]
        }]
    }
}