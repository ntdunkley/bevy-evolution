use bevy::prelude::*;
use bevy::DefaultPlugins;

mod simulation;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, simulation::SimulationPlugin))
        .run()
}
