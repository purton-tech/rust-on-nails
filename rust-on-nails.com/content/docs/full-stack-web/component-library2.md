+++
title = "Building with Components II"
description = "Building with Components"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 100
sort_by = "weight"


[extra]
toc = true
top = false
+++

Things are already starting to look more professional. Let's tidy up the  main part of the screen.

## The Finished Result

![Screenshot](../screenshot-more-components.png)

## Cards, Buttons and Forms.

Add the code below to `crates/web-pages/src/root.rs`.

```rust
use crate::{layout::{Layout, SideBar}, render};
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
```

We can now compose applications from the components in the `daisy_rsx` library. You can also look at the code for the components to see how to create your own.

