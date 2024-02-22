use std::cell::RefCell;
use std::rc::Rc;

use macroquad::{prelude::*, rand::ChooseRandom};
use rip_worldgenerator::MyWorldGen;
use olympus::channel::Channel;
use olympus::Visualizer;
use robotics_lib::{energy::Energy, event::events::Event, interface::{go, Direction}, runner::{backpack::BackPack, Robot, Runnable}, world::{coordinates::Coordinate, World}};

// Example implementation of a robot that works with olympus visualizer
struct DummyRobot{
    robot: Robot,
    channel: Rc<RefCell<Channel>> // Your robot must have a channel to comunicate with the GUI
}

impl DummyRobot {
    fn new(channel: Rc<RefCell<Channel>>) -> DummyRobot {
        DummyRobot {
            robot: Robot::default(),
            channel
        }
    }
}

impl Runnable for DummyRobot {
    fn process_tick(&mut self, world: &mut World) {
        let directions = vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
        let _ = go(self, world, directions.choose().unwrap().clone());

        // You need to call this method to update the GUI
        self.channel.borrow_mut().send_game_info(self, world);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(weather) => {
                // You need to call this method to update the GUI
                self.channel.borrow_mut().send_weather_info(weather);
            }
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_, (_, _)) => {}
            Event::TileContentUpdated(_, _) => {}
            Event::AddedToBackpack(_, _) => {}
            Event::RemovedFromBackpack(_, _) => {}
        }
    }

    fn get_energy(&self) -> &Energy { &self.robot.energy }
    fn get_energy_mut(&mut self) -> &mut Energy { &mut self.robot.energy }
    fn get_coordinate(&self) -> &Coordinate { &self.robot.coordinate }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate { &mut self.robot.coordinate }
    fn get_backpack(&self) -> &BackPack { &self.robot.backpack }
    fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.robot.backpack }
}

// You probably don't want to edit this
// But if you need to, remember that you cannot change the name of this function
fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

// This macro is Macroquad entry point
#[macroquad::main(window_conf)]
// The main() function needs to be async for the visualizer to work
async fn main() {
    // Channel
    // This is a channel used by the robot to comunicate with the GUI 
    let channel = Rc::new(RefCell::new(Channel::default()));

    // World Generator
    let world_size = 200;
    let world_generator = MyWorldGen::new_param(
        world_size,
        5,
        3,
        3,
        true,
        true,
        3,
        false,
        None
    );
    
    // Robot
    // Your robot must have channel as a field
    let robot = Box::new(DummyRobot::new(Rc::clone(&channel)));
    
    // Visualizer
    let mut visualizer = Visualizer::new(robot, world_generator, world_size, Rc::clone(&channel));
    visualizer.start().await
}