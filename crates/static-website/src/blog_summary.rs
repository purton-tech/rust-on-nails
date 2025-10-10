use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2025-02-04",
                    title: "Stop saying Kubernetes is Complicated",
                    description: "Stop saying Kubernetes is Complicated",
                    folder: "blog/kubernetes-complicated/",
                    markdown: include_str!("../content/blog/kubernetes-complicated/index.md"),
                    image: Some("/blog/kubernetes-complicated/kubernetes.webp"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
                Page {
                    date: "2025-01-28",
                    title: "Stop building React backends in Java, Python or Go",
                    description: "Stop building React backends in Java, Python or Go",
                    folder: "blog/react-backends/",
                    markdown: include_str!("../content/blog/react-backends/index.md"),
                    image: Some("/blog/react-backends/backend-army.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
                Page {
                    date: "2024-12-24",
                    title: "Rust on Nails : Version 0.1",
                    description: "Rust on Nails : Version 0.1",
                    folder: "blog/version-0.1/",
                    markdown: include_str!("../content/blog/version-0.1/index.md"),
                    image: Some("/blog/version-0.1/example-app.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
                Page {
                    date: "2025-01-09",
                    title: "SQL vs. NoSQL: A Senior Architect’s Perspective",
                    description: "SQL vs. NoSQL: A Senior Architect’s Perspective",
                    folder: "blog/sql-vs-nosql/",
                    markdown: include_str!("../content/blog/sql-vs-nosql/index.md"),
                    image: Some("/blog/sql-vs-nosql/sql-vs-nosql.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
                Page {
                    date: "2025-01-10",
                    title: "MPA vs SPA in 2025: A Senior Architect’s Perspective",
                    description: "PA vs. SPA: A Senior Architect’s Perspective",
                    folder: "blog/mpa-vs-spa/",
                    markdown: include_str!("../content/blog/mpa-vs-spa/index.md"),
                    image: Some("/blog/mpa-vs-spa/mpa-vs-spa.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
                Page {
                    date: "2025-01-15",
                    title: "Stop saying Rust is Complicated",
                    description: "Stop saying Rust is Complicated",
                    folder: "blog/rust-complicated/",
                    markdown: include_str!("../content/blog/rust-complicated/index.md"),
                    image: Some("/blog/rust-complicated/rust.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton"),
                },
            ],
        }],
    }
}
