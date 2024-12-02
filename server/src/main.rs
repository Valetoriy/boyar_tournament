use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, print_hello).run();
}

fn print_hello() {
    println!("Glinomecivo")
}
