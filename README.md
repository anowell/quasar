# Quasar

An experimental rust-to-{wasm,asmjs} frontend framework.

---

Supermassive black holes exist at the center of most observed galaxies, but there is much about them that remains a mystery. It is believed that rapidly adding sufficient matter to a supermassive black hole's accretion disk results in becoming a quasar that emits enormous amounts of electromagnetic energy as matter via astrophysical jets perpendicular to the black hole's spin. These jets can emit matter at nearly lightspeed and stretch hundreds of thousands of light years across a galaxy.

WASM is at the center of an upcoming shift in web development, but there is still much about that shift that remains a mystery. Some believe Rust, as an early mover, and with zero-cost abstractions is well-positioned to invest in bytecode that spins up on the event loop. It may be possible for Rust to power the fastest applications on the web, becoming highly visible across the frontend ecosystem for years to come.

Oh, and black hole's form from the collapse of a core of iron.. you know, the only element that rusts.

<img title="Artist's rendition of a quasar" src="https://upload.wikimedia.org/wikipedia/commons/3/38/Artist%27s_rendering_ULAS_J1120%2B0641.jpg" width="320">

---

## Current Status

Everything is experimental, half-baked, full of caveats, and subject to change.

That said:
- templating engines are swappable with examples using [mustache](https://crates.io/crates/mustache)(default) and [maud](https://crates.io/crates/maud).
- templates can one-way bind to both data and properties of the node they are rendered into
- event handlers can be assigned to views and used to update bound data or more directly the view itself

A basic example might include an HTML file like this:

```html
<html>
  <body>
    <Reverser name="Malcom Reynolds"></Reverser>
    <Reverser name="Shepherd Book"></Reverser>
  </body>
</html>
```

You can bind and update data with a snippet like this:

```rust
fn main() {
    let mut qdom = quasar::init();

    let my_widget = Component {
        data: ReverseData{
            message: "Hello World".to_owned()
        },
        props: vec!["name"],
        template: compile_str(r##"
            <p>{{ props.name }}, {{ message }}</p>
            <button>Reverse Message</button>
        "##).expect("failed to compile my_widget template")
    };

    let views = qdom.render(my_widget, "Reverser");
    views.on(EventType::Click, |evt| {
        println!("Reverser clicked!!!");
        evt.component.data.message = evt.component.data.message.chars().rev().collect();
    });
}
```

Skim through the [`examples`](https://github.com/anowell/quasar/tree/master/examples) directory to get a sense of how to use it today.

## What's next?

I'm still working to better understand what works and what's missing in [webplatform](https://github.com/tcr/rust-webplatform).
Here are some overarching questions that are guiding this experimentation right now:

- Can Quasar achieve a level of abstractions that feel comparable to modern Javascript frameworks?
- What might it look like to have "isomorphic" rust, where the same rendering code can run both client and server side?
- What might a modular, trait-based templating engine look like?

Admittedly Quasar is absent any perf goals, but more importantly, Quasar lacks a compelling vision for why Quasar would be better than X, so I'll probably ask myself "what problem is Quasar really solving?" multiple times throughout this experimentation.
