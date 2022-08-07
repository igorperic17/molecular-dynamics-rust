mod atom;
mod math;
mod particle;
mod simulation;

fn main() {
    println!("Constructing the simulation...");

    let steps = 50000;

    // create a simulation with deltaT = 1fs and temperature = 500 K
    let mut simulation = simulation::Simulation::new(1.0, 500.0);

    let hydrogen = atom::Atom::create_hidrogen(math::Vec3::new(0.0, 0.0, 0.0));
    simulation.add_atom(&hydrogen);

    simulation.run(steps);
}
