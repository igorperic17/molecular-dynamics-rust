mod atom;
mod math;
mod particle;
mod simulation;

extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Matrix, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};

const SIMULATION_SCALE: f32 = 10e8;

struct AppState {
    nodes: Vec<SceneNode>,
    simulation: simulation::Simulation,
}

impl AppState {
    pub fn new(window: &mut kiss3d::window::Window, simulation: simulation::Simulation) -> Self {
        // init all paticle nodes in the scene
        let mut nodes: Vec<SceneNode> = Vec::<SceneNode>::new();
        for particle in &simulation.particles {
            let mut particle_node =
                window.add_sphere(particle::Particle::get_radius(particle.particle_type));
            let (r, g, b) = particle::Particle::get_color(particle.particle_type);
            particle_node.set_color(r, g, b);
            particle_node.append_translation(&Translation3::new(
                particle.pos.x * SIMULATION_SCALE,
                particle.pos.y * SIMULATION_SCALE,
                particle.pos.z * SIMULATION_SCALE,
            ));
            nodes.push(particle_node);
        }

        Self {
            nodes: nodes,
            simulation: simulation,
        }
    }
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.simulation.step();
        for (i, particle) in self.simulation.particles.iter().enumerate() {
            self.nodes[i].append_translation(&Translation3::new(
                particle.pos.x * SIMULATION_SCALE,
                particle.pos.y * SIMULATION_SCALE,
                particle.pos.z * SIMULATION_SCALE,
            ));
        }
    }
}

fn main() {
    // let steps = 50000;

    // create a simulation with deltaT = 1fs and temperature = 500 K
    let mut simulation = simulation::Simulation::new(1.0, 500.0);

    let hydrogen = atom::Atom::create_hidrogen(math::Vec3::new(0.0, 0.0, 0.0));
    simulation.add_atom(&hydrogen);

    let window: &mut Window = &mut Window::new("Kiss3d: wasm example");
    window.set_light(Light::StickToCamera);
    // let mut c = window.add_sphere(proton_render_radius);

    // c.set_color(1.0, 1.0, 1.0);
    // c.set_local_translation(Translation3::new(0.0, 0.0, 10.0));

    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = &mut AppState::new(window, simulation);

    while window.render() {
        state.step(window);
    }

    println!("Constructing the simulation...");
}
