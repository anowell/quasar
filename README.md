# Quasar

An experimental rust-to-{wasm,asmjs} frontend framework.

---

Supermassive black holes exist at the center of most observed galaxies, but there is much about them that remains a mystery. It is believed that rapidly adding sufficient matter to a supermassive black hole's accretion disk results in becoming a quasar that emits enormous amounts of electromagnetic energy as matter via astrophysical jets perpendicular to the black hole's spin. These jets can emit matter at nearly lightspeed and stretch hundreds of thousands of light years across a galaxy.

WASM is at the center of an upcoming shift in web development, but there is still much about that shift that remains a mystery. Some believe Rust, as an early mover, and with zero-cost abstractions is well-positioned to invest in bytecode that spins up on the event loop. It may be possible for Rust to power the fastest applications on the web, becoming highly visible across the frontend ecosystem for years to come.

Oh, and black hole's form from the collapse of a core of iron.. you know, the only element that rusts.

<img title="Artist's rendition of a quasar" src="https://upload.wikimedia.org/wikipedia/commons/3/38/Artist%27s_rendering_ULAS_J1120%2B0641.jpg" width="320">

---

## How it works

Everything is experimental, half-baked, full of caveats, and subject to change. But currently:

- **Template engines** are swappable. There are [`examples`](https://github.com/anowell/quasar/tree/master/examples) using [mustache](https://crates.io/crates/mustache)(default) and [maud](https://crates.io/crates/maud). But replacing the template engine is just a matter of implementing the `Renderable` trait.
- **Components** are the combination of data with a template or other rendering process - really anything that implements `Renderable`. Quasar takes ownership of your components when binding them to the DOM and makes the data available to your event handlers via `data()` and `data_mut()` methods. In general, methods that mutate the component will result in re-rendering it (TBD: at the end of the event handler or at next tick). Note, component data is local to the component and not shareable outside your component.
- **Views** are the result of one-way binding of a component to the DOM. You can also attach event listeners to views. Note, that currently rendering a view uses the destructive `innerHtml = ...` approach, which kills DOM state like input focus, so eventually some sort of DOM diffing/patching or virtual DOM solution will become pretty important.
- **State** or the shared app data is also available to event handlers. It is partitioned by a key (and by `TypeId`), and any attempt to read a shared data partition (calling `data(key)`) automatically registeres your view as an observer of that data partion (to-be implemented). Any attempt to write to an app data partition (calling `data_mut(key)`) will automatically add all observer views for that data partition to the re-render queue (TBD: processed at the end of the event handler or at the next tick).

A couple basic principles are beginning to emerge. With Quasar...
- **it should be impossible in safe Rust to update state in your application without also updating views.**
- **all your app and component state are statically typed in data structures of your choosing**

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
    let mut app = quasar::init();

    let component = Component {
        props: vec![],
        data: CounterData {
            count: 0
        },
        template: compile_str(r##"
            <p>Count: {{count}}</p>
            <button>+1</button>
        "##).expect("failed to compile counter template")
    };

    app.bind(component, "#counter")
        .on(EventType::Click, |mut evt| {
            evt.view.data_mut().count += 1;
        });

    app.spin();
}
```

See the [`examples`](https://github.com/anowell/quasar/tree/master/examples) directory to get a sense of how it works today.

## What's next?

I'm still working to better understand what works and what's missing in [webplatform](https://github.com/tcr/rust-webplatform).
Here are some overarching questions that are guiding this experimentation right now:

- Can Quasar achieve a level of abstractions that feel comparable to modern Javascript frameworks? (I believe some macros could allow it to rival the declarative syntax of some other frameworks.)
- What might it look like to have "isomorphic" rust, where the same rendering code can run both client and server side?
- How can I leverage the type system to achieve more flexible and/or more robust frontend development? (e.g. trait-based templating, leveraging immutable vs mutable access as a gate for identifying views that observer or mutate specific data.)

Admittedly Quasar is absent any perf goals at this time. Quasar also lacks a clear vision for why Quasar would be "better than X", so I'll probably ask myself "what problem is Quasar really solving?" multiple times throughout this experimentation.
