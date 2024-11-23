use bevy::prelude::*;

#[derive(Event)]
struct MyEvent;

fn main() {
    App::new().init_resource::<Events<MyEvent>>();
}
