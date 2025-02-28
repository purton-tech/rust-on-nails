# HTMX and Interactivity

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

In the `crates/web-pages/src/root.rs` update the form and give it a [hx-boost](https://htmx.org/attributes/hx-boost/) attribute.

The form will now use Ajax to communicate with the back end and we get a smoother experience on the front end.

```rust
...
form {
    // 👇 Boost the form
    "hx-boost": "true", 
...
```