use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Setting Up".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Introduction",
                        description: "Install Stack and apply your first StackApp",
                        folder: "docs/",
                        markdown: include_str!("../content/docs/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Stack Architecture",
                        description: "How the operator, CRDs, and shared services fit together",
                        folder: "docs/architecture/",
                        markdown: include_str!("../content/docs/architecture/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Keycloak Operator",
                        description: "What stack init installs and how Keycloak powers OAuth2",
                        folder: "docs/keycloak-operator/",
                        markdown: include_str!("../content/docs/keycloak-operator/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "PostgreSQL Operator",
                        description: "How CloudNativePG is installed and used per StackApp",
                        folder: "docs/postgres-operator/",
                        markdown: include_str!("../content/docs/postgres-operator/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
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
            Category {
                name: "Frameworks".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Rails on Kubernetes",
                        description: "Containerise a Rails app and ship it with Stack",
                        folder: "docs/framework/",
                        markdown: include_str!("../content/docs/framework/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Flask on Kubernetes",
                        description: "Build a Python/Flask container and deploy it with Stack",
                        folder: "docs/framework/flask/",
                        markdown: include_str!("../content/docs/framework/flask.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
