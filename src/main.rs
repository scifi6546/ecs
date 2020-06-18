use nalgebra::Vector3;
#[macro_use]
mod lib;
#[macro_use]
use lib::*;
mod PlanetSim{
use nalgebra::Vector3;
    create_entity!(mass:f64,Position:Vector3<f64>,Velocity:Vector3<f64>);

}
fn main() {
    println!("Hello, world!");
}
