# Web Islands (Webassembly)

The [web islands pattern](https://www.patterns.dev/vanilla/islands-architecture/) allows to keep most of the functionality server side rendered but we can add some interactivity in the front end when needed.

To do this we'll follow the guidelines in [Reasonable System for JavaScript Structure](https://ricostacruz.com/rsjs/) and apply those to Rust.

## Using an alert component example.

To follow the RSJS guidelines, we will use data attributes, that is, HTML attributes that begin with data-, a standard feature of HTML, to indicate that our HTML is a counter component. We will then write Rust to use an attribute selector that looks for the data-counter attribute as the root element in our counter component and wires in the appropriate event handlers and logic. 

Let's make a very simple component that show an alert on click.

```html
<div class="text-center text-sm" data-alert="">You can place items at the bottom</div>
```

We've already included this in our `crates/web-pages/layout.rs`, so let's make it interactive.

## A new web-islands (client side rendering) crate

```sh
cargo new --lib crates/web-islands
```

You can let Cargo wire up the dependencies for you before editing the manifest further:

```sh
cd crates/web-csr
cargo add wasm-bindgen
cargo add web-sys --features Document,Element,Event,HtmlElement,Node,Window,console
cargo add console_error_panic_hook --optional
cargo add wasm-bindgen-test --dev
```

That takes care of the version pins shown below; you will still need to add the `[lib]` `crate-type`, feature declarations, and release profile tweaks manually.

## Cargo.toml

```toml
[package]
name = "web-csr"
version = "0.1.0"
edition = "2024"


[lib]
crate-type = ["cdylib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.84"
web-sys = { version = "0.3.76", features = ['Document', 'Element', 'Event', 'HtmlElement', 'Node', 'Window', 'console'] }

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.7", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.34"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

## Our first component

This component attaches to any element with the attribute `data-alert` and shows an alert if the element is clicked.

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, window};

// A simple helper function to get the document from the global window.
fn document() -> Document {
    window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document")
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn hydrate() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let doc = document();
    // Query for the button element with `data-alert` attribute.
    let button: Option<Element> = doc.query_selector("[data-alert]").ok().flatten();

    if let Some(btn_el) = button {
        // Convert to an HtmlElement to attach event listener easily.
        let btn = btn_el.dyn_into::<HtmlElement>()?;

        // Closure for the event listener.
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
            // Show the alert when button is clicked.
            window()
                .unwrap()
                .alert_with_message("Alert triggered!")
                .unwrap();
        });

        // Add the click event listener to the button.
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Keep the closure alive for the lifetime of the program.
    } else {
        web_sys::console::log_1(&"No element with data-alert found!".into());
    }

    Ok(())
}
```

## Update our Justfile

Add an entry `wasm`

```justfile
islands:
    cargo watch \
      -w crates/web-islands \
      -s 'cargo build -p web-islands --target wasm32-unknown-unknown --release && \
          wasm-bindgen \
            target/wasm32-unknown-unknown/release/web_islands.wasm \
            --target web \
            --out-dir crates/web-assets/dist'
```

Now run `just islands` and some wasm files will be created in the `crates/web-assets/dist` folder.

## Add webassembly to our layout

Go into `crates/web-pages/layout.rs` and uncomment the lines below

```rust
//web_assembly: (
//    web_assets::statics::web_islands_js.name.into(),
//    web_assets::statics::web_islands_bg_wasm.name.into()
//),
```

We need to update our `watch` so that it sees the webassembly files when they are placed in `dist` update the Justfile `watch` section as below.

```Justfile 
watch:
    mold -run cargo watch --workdir /workspace/ \
      -w crates/web-server \
      -w crates/web-pages \
      -w crates/clorinde \
      -w crates/web-assets/dist \
      --no-gitignore \
      -x "run --bin web-server"
```

Our layout will now serve our webassembly files.
