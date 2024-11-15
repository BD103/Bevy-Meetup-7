use bevy::prelude::*;

fn main() {
    // SNIP...
    #[derive(Event)]
    struct MyEvent;

    App::new().init_resource::<Events<MyEvent>>().run();
    // ...SNIP
}
