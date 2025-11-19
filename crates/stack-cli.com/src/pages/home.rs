use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navigation::Section;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn home_page() -> String {
    let install_script = r#"export STACK_VERSION=v1.3.31
curl -OL https://github.com/purton-tech/rust-on-nails/releases/download/${STACK_VERSION}/stack-cli \
  && chmod +x ./stack-cli \
  && sudo mv ./stack-cli /usr/local/bin/stack"#;

    let features = vec![
        (
            "Managed PostgreSQL",
            "Provision CloudNativePG with dedicated storage, secure credentials, and separate roles for applications, migrations, and read-only traffic.",
        ),
        (
            "Identity & Access Control",
            "Deploy Keycloak with dedicated storage, wire it to OAuth2 Proxy, and enforce single sign-on across your apps.",
        ),
        (
            "Ingress & Connectivity",
            "Bootstrap NGINX ingress, lock it down with network policies, and expose services safely through Cloudflare tunnels.",
        ),
        (
            "Secure Namespace Defaults",
            "Apply OAuth2 Proxy plus NetworkPolicies so every namespace starts with locked-down ingress and hardened traffic paths.",
        ),
        (
            "Developer Tooling",
            "Ship PGAdmin, MailHog, and other helper services to inspect data, test email flows, and unblock your team instantly.",
        ),
    ];

    let benefits = vec![
        (
            "Accelerate platform setup",
            "Import a battle-tested set of operators with one command instead of stitching together manifests by hand.",
        ),
        (
            "Stay secure by default",
            "Stack templates secrets, OAuth flows, and network policies so every environment starts hardened.",
        ),
        (
            "Standardise every cluster",
            "Roll the same blueprint into dev, staging, and production to reduce snowflake drift and unexpected outages.",
        ),
        (
            "Focus on your applications",
            "Let Stack own the infrastructure glue so your team can ship business features instead of wrangling YAML.",
        ),
    ];

    let page = rsx! {
        Layout {
            title: "Stack",
            description: "Stack turns Kubernetes into a self-hosted PaaS so you can deploy apps without assembling operators by hand.",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "p-5 mt-16 mx-auto max-w-5xl",
                Hero {}

            }

            section {
                id: "install-stack",
                class: "mt-20 px-5",
                div {
                    class: "max-w-4xl mx-auto bg-base-200 border border-base-300 rounded-2xl p-8",
                    h2 {
                        class: "text-3xl font-semibold text-center",
                        "Install the Stack CLI"
                    }
                    p {
                        class: "mt-4 text-center text-base-content/80",
                        "Download the CLI and bring the Stack platform into any Kubernetes cluster with a single command."
                    }
                    pre {
                        class: "mt-6 bg-black text-white text-sm rounded-xl p-5 overflow-x-auto",
                        code {
                            class: "language-bash",
                            "{install_script}"
                        }
                    }
                }
            }

            section {
                class: "mt-20 px-5",
                div {
                    class: "max-w-6xl mx-auto",
                    h2 {
                        class: "text-3xl font-semibold text-center",
                        "Everything a delivery team needs"
                    }
                    p {
                        class: "mt-4 text-center text-base-content/80",
                        "Stack installs a curated set of services so your applications are ready for production from day one."
                    }
                    div {
                        class: "mt-10 grid grid-cols-1 md:grid-cols-2 gap-6",
                        for (title, description) in features.iter() {
                            div {
                                class: "border border-base-300 rounded-2xl bg-base-200 p-6",
                                h3 {
                                    class: "text-xl font-semibold",
                                    "{title}"
                                }
                                p {
                                    class: "mt-2 text-base-content/80",
                                    "{description}"
                                }
                            }
                        }
                    }
                }
            }

            section {
                class: "mt-20 mb-24 px-5",
                div {
                    class: "max-w-5xl mx-auto",
                    h2 {
                        class: "text-3xl font-semibold text-center",
                        "Why teams choose Stack"
                    }
                    div {
                        class: "mt-10 grid grid-cols-1 md:grid-cols-2 gap-6",
                        for (title, description) in benefits.iter() {
                            div {
                                class: "border border-base-300 rounded-2xl bg-base-100 p-6",
                                h3 {
                                    class: "text-xl font-semibold",
                                    "{title}"
                                }
                                p {
                                    class: "mt-2 text-base-content/80",
                                    "{description}"
                                }
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
