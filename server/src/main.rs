use bevy::prelude::*;

fn main() {
    App::new().add_systems(print_hello).run();
}

fn print_hello() {
    println!("Glinomecivo")
}
