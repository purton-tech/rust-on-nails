use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Setting Up".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Introduction",
                    description: "Install Stack and apply your first StackApp",
                    folder: "docs/",
                    markdown: include_str!("../content/docs/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "Connectivity".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Cloudflare Tunnels",
                    description: "Expose Stack namespaces via quick or authenticated tunnels",
                    folder: "docs/cloudflare/",
                    markdown: include_str!("../content/docs/cloudflare/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
        ],
    }
}
