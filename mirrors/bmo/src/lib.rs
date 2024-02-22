use std::cell::RefCell;
use std::rc::Rc;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use olympus::channel::Channel;
use robotics_lib::interface::{Direction, go};
use robotics_lib::utils::go_allowed;
use macroquad::rand::ChooseRandom;

pub struct BMO {
    robot: Robot,
    channel: Rc<RefCell<Channel>>
}

impl BMO {
    pub fn new(channel: Rc<RefCell<Channel>>) -> BMO {
        BMO {
            robot: Robot::default(),
            channel
        }
    }
}

impl Runnable for BMO {
    fn process_tick(&mut self, world: &mut World) {
        let directions = vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right
        ];
        let dir = directions.choose().unwrap();

        match go_allowed(self, world, dir) {
            Ok(_) => {
                let _ = go(self, world, dir.clone());
            },
            Err(_) => ()
        }

        // You need to call this method to update the GUI
        self.channel.borrow_mut().send_game_info(self, world);
    }


    fn handle_event(&mut self, event: Event) {

        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(weather) => {
                // Update the GUI
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
