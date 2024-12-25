use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{console, window, Event, HtmlElement};

#[wasm_bindgen]
pub fn load() -> Result<(), JsValue> {
    let window = window().ok_or("no global `window` exists")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;

    // Find the section with class "counter" and "data-counter"
    let section: HtmlElement = document
        .query_selector("section.counter[data-counter]")?
        .ok_or(JsValue::from_str("No section found"))?
        .dyn_into()?;

    // Initialize the data-counter attribute to "0"
    section.set_attribute("data-counter", "0")?;

    // Find the increment button
    let increment_button: HtmlElement = document
        .query_selector("button[data-counter-increment]")?
        .ok_or(JsValue::from_str("No increment button found"))?
        .dyn_into()?;

    // Set initial button text
    increment_button.set_text_content(Some("Increment (0)"));

    // Wrap the elements in Rc so we can clone them into closures
    let section_rc = Rc::new(section);
    let button_rc = Rc::new(increment_button);

    // Clone the Rc references for the closure
    let section_for_closure = section_rc.clone();
    let button_for_closure = button_rc.clone();

    let closure = Closure::wrap(Box::new(move |_: Event| {
        console::log_1(&"DGot a click".into());
        // Get the current count from the section's data-counter attribute
        if let Some(current_val_str) = section_for_closure.get_attribute("data-counter") {
            let current_val: u32 = current_val_str.parse().unwrap_or(0);
            let new_val = current_val + 1;

            // Update the section's data-counter attribute
            if let Err(err) =
                section_for_closure.set_attribute("data-counter", &new_val.to_string())
            {
                web_sys::console::error_1(&err);
            }

            // Update the button text
            button_for_closure.set_text_content(Some(&format!("Increment ({})", new_val)));
        }
    }) as Box<dyn FnMut(_)>);

    // Add the event listener to the button
    button_rc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;

    // Prevent the closure from being dropped
    closure.forget();

    Ok(())
}
