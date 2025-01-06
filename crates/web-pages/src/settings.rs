use crate::{
    layout::{Layout, SideBar},
    render,
};
use daisy_rsx::*;
use db::User;
use dioxus::prelude::*;
use web_assets::files::favicon_svg;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            selected_item: SideBar::Users,
            BlankSlate {
                heading: "Welcome To Your Application",
                visual: favicon_svg.name,
                description: "This is just the beginning",
            }
            Card {
                class: "card-bordered mt-12 has-data-table",
                CardHeader {
                    class: "p-3 border-b",
                    title: "Users"
                }
                CardBody {
                    class: "p-0",
                    table {
                        class: "table table-sm",
                        thead {
                            tr {
                                th { "ID" }
                                th { "Email" }
                            }
                        }
                        tbody {
                            for user in users {
                                tr {
                                    td {
                                        strong {
                                            "{user.id}"
                                        }
                                    }
                                    td {
                                        "{user.email}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }
    };

    render(page)
}
