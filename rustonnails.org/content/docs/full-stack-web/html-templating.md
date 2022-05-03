+++
title = "HTML Templating"
description = "HTML Templating"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 70
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

Because Rust supports macros we can create a DSL to handle our templating needs. This means we can leverage the Rust we already know to create loops and if statements.

We will use [markup.rs](https://github.com/utkarshkukreti/markup.rs). Add the following to your `app/src/Cargo.toml`.

```toml
markup = "0"
```

**Markup** templating syntax looks like what we see below. We can re-use our existing Rust knowledge for conditions and loops and get compile time help at the same time.

```rust
markup::define! {
    Page<'a>(user: &'a User, posts: &'a [Post]) {
        @markup::doctype()
        html {
            head {
                title { "Hello " @user.name }
            }
            body {
                #main.container {
                    @for post in *posts {
                        div#{format!("post-{}", post.id)}["data-id" = post.id] {
                            .title { @post.title }
                        }
                    }
                }
                @Footer { name: &user.name, year: 2020 }
            }
        }
    }

    Footer<'a>(name: &'a str, year: u32) {
        "(c) " @year " " @name
    }
}
```