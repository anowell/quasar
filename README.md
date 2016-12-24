# Quasar

An experimental rust-to-{wasm,asmjs} frontend framework. 

---

Supermassive black holes exist at the center of most observed galaxies, but there is much about them that remains a mystery. It is believed that rapidly adding sufficient matter to a supermassive black hole's accretion disk results in becoming a quasar that emits enormous amounts of electromagnetic energy as matter via astrophysical jets perpendicular to the black hole's spin. These jets can emit matter at nearly lightspeed and stretch hundreds of thousands of light years across a galaxy.

WASM is at the center of an upcoming shift in web development, but there is still much about that shift that remains a mystery. Some believe Rust, as an early mover, and with zero-cost abstractions is well-positioned to invest in bytecode that runs on the spinning event loop. It may be possible for Rust to power the fastest applications on the web, becoming highly visible across the frontend ecosystem for years to come.

Oh, and black hole's form from the collapse of a core of iron.. you know, the only element that rusts.

<img title="Artist's rendition of a quasar" src="https://upload.wikimedia.org/wikipedia/commons/3/38/Artist%27s_rendering_ULAS_J1120%2B0641.jpg" width="320">

---

## Current Status

Only a couple things work, and everything about this is subject to change. That said, given an HTML file like this:

```html
<html>
  <body>
    <Reverser></Reverser>
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
        template: compile_str(r##"
            <p>{{ message }}</p>
            <button>Reverse Message</button>
        "##).expect("failed to compile template")
    };

    let view = qdom.render(my_widget, "Reverser");
    view.on(EventType::Click, |ref mut data| {
        println!("on click called");
        data.message = data.message.chars().rev().collect::<String>();
    });
}
```

That's all for now.
