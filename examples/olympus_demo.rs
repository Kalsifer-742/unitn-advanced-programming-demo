use std::{cell::RefCell, env};
use std::rc::Rc;
use ai::MyRobot;
use bmo::BMO;
use macroquad::prelude::*;
use midgard::params::WorldGeneratorParameters;
use midgard::WorldGenerator;
use olympus::channel::Channel;
use rip_worldgenerator::MyWorldGen;
use olympus::Visualizer;
use robotics_lib::runner::Runnable;
use robotics_lib::world::world_generator::Generator;

fn get_robot(selection: String, channel: Rc<RefCell<Channel>>) -> Box<dyn Runnable> {
    let robot = if selection == "bmo" {
        Box::new(
            BMO::new(channel)
        ) as Box::<dyn Runnable>
    } else {
        Box::new(
            MyRobot::new(channel)
        ) as Box::<dyn Runnable>
    };

    robot
}

struct GeneratorWrapper {
    generator: Box<dyn Generator>
}
impl Generator for GeneratorWrapper {
    fn gen(&mut self) -> robotics_lib::world::world_generator::World { self.generator.gen() }
}
impl GeneratorWrapper {
    pub fn new(generator: Box<dyn Generator>) -> Self { Self { generator } }
}

fn get_world_generator(selection: String) -> (GeneratorWrapper, usize) {
    let world_size = 200;

    let generator = if selection == "rip" {
        Box::new(
            MyWorldGen::new_param(
                world_size, 5, 3, 3,
                true, true, 3, false,
                None
            )
        ) as Box::<dyn Generator>
    } else {
        let params = WorldGeneratorParameters {
            world_size,
            amount_of_streets: Some(0.7),
            ..Default::default()
        };
        Box::new(
            WorldGenerator::new(params)
        ) as Box::<dyn Generator>
    };
    
    (GeneratorWrapper::new(generator), world_size)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let robot_selection = &args[1];
    let world_generator_selection = &args[2];

    let channel = Rc::new(RefCell::new(Channel::default()));
    let robot = get_robot(robot_selection.to_string(), Rc::clone(&channel));
    let (world_generator, world_size) = get_world_generator(world_generator_selection.to_string());
    
    let mut visualizer = Visualizer::new(robot, world_generator, world_size, Rc::clone(&channel));
    visualizer.start().await
}