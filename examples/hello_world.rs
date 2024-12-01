use leptos::{ev::keydown, prelude::*};
use leptos_use::use_event_listener;
use lerni::*;

/// Hello world example.
#[component]
pub fn HelloWorld() -> impl IntoView {
    view! {
        <SlideShow>
            <Slide background_color=Color::MistyRose>
                <Label angle=10>"Hello →"</Label>
            </Slide>
            <Slide background_color=Color::PaleGreen>
                <Label angle=-10>"← World!"</Label>
            </Slide>
            <Counter />
        </SlideShow>
    }
}

/// Counter component.
#[component]
pub fn Counter() -> impl IntoView {
    let (counter, set_counter) = signal(0);

    let node_ref = NodeRef::new();
    _ = use_event_listener(document().body(), keydown, move |e| {
        if is_active_slide(node_ref) {
            if e.key() == "Enter" {
                set_counter.set(counter.get() + 1);
            } else if e.key() == "Escape" && counter.get() > 0 {
                set_counter.set(counter.get() - 1);
            }
        }
    });

    view! {
        <Slide node_ref=node_ref>
            <Label>
                "Counter (press 'Enter' and 'Escape' to change): " {move || counter.get()}
            </Label>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(HelloWorld);
}
