+++
title = "HTMX and Interactivity"
description = "HTMX and Interactivity"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 110
sort_by = "weight"


[extra]
toc = true
top = false
+++

[HTMX](https://htmx.org/) let's us add interactivity to our applications without writing any Javascript.

## Installing HTMX

```sh
mkdir crates/web-assets/js && curl https://unpkg.com/htmx.org@2.0.3 -oL crates/web-assets/js/htmx-2.0.3.js
```

Update the `crates/web-assets/build.rs` file to include the new folder.

```rust
use cache_busters::generate_static_files_code;
use std::env;
use std::path::PathBuf;

fn main() {
    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Example of multiple asset directories
    let asset_dirs = vec![
        PathBuf::from("./js"),
        PathBuf::from("./images"),
        PathBuf::from("./dist"),
    ];

    generate_static_files_code(&static_out_dir, &asset_dirs).unwrap();
}
```

And add the JS to our `layout.rs` in `crates/web-pages/src/layour.rs`.

```rust
BaseLayout {
    title,
    stylesheets: vec![tailwind_css.name.to_string()],

    // 👇 We added the HTML js.
    js_href: htmx_2_0_3_js.name,
```

## Form Submission Without Page Refresh