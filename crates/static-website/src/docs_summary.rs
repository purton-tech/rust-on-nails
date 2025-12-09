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
                        description: "Introducing Nails",
                        folder: "docs/",
                        markdown: include_str!("../content/docs/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Development Environment as Code",
                        description: "Development Environment as Code",
                        folder: "docs/ide-setup/dev-env-as-code",
                        markdown: include_str!(
                            "../content/docs/ide-setup/dev-env-as-code/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "The Database".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "The Database",
                        description: "The Database",
                        folder: "docs/database-part-1/the-database/",
                        markdown: include_str!(
                            "../content/docs/database-part-1/the-database/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Database Schema Migrations",
                        description: "Database Schema Migrations",
                        folder: "docs/database-part-1/database-migrations/",
                        markdown: include_str!(
                            "../content/docs/database-part-1/database-migrations/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Database Access",
                        description: "Database Access",
                        folder: "docs/database-part-1/database-access/",
                        markdown: include_str!(
                            "../content/docs/database-part-1/database-access/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Common Schemas".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Auth Schema",
                        description: "Reusable authentication schema snippets",
                        folder: "docs/common-schemas/auth/",
                        markdown: include_str!(
                            "../content/docs/common-schemas/auth/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Teams Schema",
                        description: "Collaborative teams modeled inside the auth schema",
                        folder: "docs/common-schemas/teams/",
                        markdown: include_str!(
                            "../content/docs/common-schemas/teams/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "RBAC Schema",
                        description: "Roles, permissions, and helpers that build on auth.me()",
                        folder: "docs/common-schemas/rbac/",
                        markdown: include_str!(
                            "../content/docs/common-schemas/rbac/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Web Development".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "The Web Server and Routing",
                        description: "The Web Server and Routing",
                        folder: "docs/full-stack-web/web-server/",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/web-server/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Building Pages",
                        description: "Building Pages",
                        folder: "docs/full-stack-web/web-pages/",
                        markdown: include_str!("../content/docs/full-stack-web/web-pages/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Assets and Cache Busting",
                        description: "Assets and Cache Busting",
                        folder: "docs/full-stack-web/cache-busting",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/cache-busting/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Forms and Actions",
                        description: "Forms and Actions",
                        folder: "docs/full-stack-web/forms",
                        markdown: include_str!("../content/docs/full-stack-web/forms/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Adding Some Style",
                        description: "Adding Some Style",
                        folder: "docs/full-stack-web/styling",
                        markdown: include_str!("../content/docs/full-stack-web/styling/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Building with Components I",
                        description: "Building with Components I",
                        folder: "docs/full-stack-web/component-library",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/component-library/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Building with Components II",
                        description: "Building with Components II",
                        folder: "docs/full-stack-web/component-library2",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/component-library2/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "HTMX and Interactivity",
                        description: "HTMX and Interactivity",
                        folder: "docs/full-stack-web/htmx",
                        markdown: include_str!("../content/docs/full-stack-web/htmx/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "More Interactivity (Webassembly)",
                        description: "More Interactivity (Webassembly)",
                        folder: "docs/full-stack-web/more-interactivity",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/more-interactivity/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "More Interactivity (Typescript)",
                        description: "More Interactivity (Typescript)",
                        folder: "docs/full-stack-web/more-interactivity-ts",
                        markdown: include_str!(
                            "../content/docs/full-stack-web/more-interactivity-ts/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Adding an API".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Integrating gRPC",
                        description: "Integrating gRPC",
                        folder: "docs/api/grpc",
                        markdown: include_str!("../content/docs/api/grpc/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "gRPC Web",
                        description: "gRPC Web",
                        folder: "docs/api/grpc-web",
                        markdown: include_str!("../content/docs/api/grpc-web/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Continous Integration".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Build Our Containers",
                        description: "Build Our Containers",
                        folder: "docs/continuous-integration/build",
                        markdown: include_str!(
                            "../content/docs/continuous-integration/build/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Running CI on Github",
                        description: "Running CI on Github",
                        folder: "docs/continuous-integration/github",
                        markdown: include_str!(
                            "../content/docs/continuous-integration/github/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Integration Testing",
                        description: "Integration Testing",
                        folder: "docs/continuous-integration/integration-testing",
                        markdown: include_str!(
                            "../content/docs/continuous-integration/integration-testing/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Auxiliary Services".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Sending Email",
                        description: "Sending Email",
                        folder: "docs/auxiliary-services/sending-email",
                        markdown: include_str!(
                            "../content/docs/auxiliary-services/sending-email/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Authentication",
                        description: "Authentication",
                        folder: "docs/auxiliary-services/authentication",
                        markdown: include_str!(
                            "../content/docs/auxiliary-services/authentication/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Database Backups and Development",
                        description: "Database Backups and Development",
                        folder: "docs/auxiliary-services/database-backups",
                        markdown: include_str!(
                            "../content/docs/auxiliary-services/database-backups/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Continuous Deliver / Deployment".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Deployed Application Topology",
                        description: "Understand the operator-managed workloads",
                        folder: "docs/continous-deployment/deployment-architecture",
                        markdown: include_str!(
                            "../content/docs/continous-deployment/deployment-architecture/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Deploying to Kubernetes",
                        description: "Deploying to Kubernetes",
                        folder: "docs/continous-deployment/kubernetes",
                        markdown: include_str!(
                            "../content/docs/continous-deployment/kubernetes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Infrastructure as Code",
                        description: "Infrastructure as Code",
                        folder: "docs/continous-deployment/infra-as-code",
                        markdown: include_str!(
                            "../content/docs/continous-deployment/infra-as-code/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Cloudflare as Ingress",
                        description: "Cloudflare as Ingress",
                        folder: "docs/continous-deployment/cloudflare",
                        markdown: include_str!(
                            "../content/docs/continous-deployment/cloudflare/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Database Part II".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Authorization",
                    description: "Authorization",
                    folder: "docs/database-part-2/authorization",
                    markdown: include_str!(
                        "../content/docs/database-part-2/authorization/index.md"
                    ),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
        ],
    }
}
