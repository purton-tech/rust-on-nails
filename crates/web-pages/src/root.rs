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

            Card {
                class: "card-bordered mt-12",
                CardHeader {
                    class: "p-3 border-b",
                    title: "Add User"
                }
                CardBody {
                    class: "p-3",
                    form {
                        "hx-boost": "true",
                        class: "flex flex-col",
                        action: "/new_user",
                        method: "POST",

                        Input {
                            input_type: InputType::Email,
                            placeholder: "e.g. ian@test.com",
                            help_text: "Please enter an email address",
                            required: true,
                            label: "Email",
                            name: "email"
                        }
                        Button {
                            class: "mt-4",
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Submit"
                        }
                    }
                }
            }
        }
    };

    render(page)
}
