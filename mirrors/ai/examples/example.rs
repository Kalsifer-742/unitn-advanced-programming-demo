use ai::*;
use olympus::{channel::Channel, *};
use asfalt_inator::AsfaltInator;
use cargo_commandos_lucky::lucky_function::lucky_spin;
//use op_map::op_pathfinding::
use ragnarok::GuiRunner;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{go, look_at_sky, one_direction_view, put, robot_map, robot_view, teleport, where_am_i, Direction, Tools},
    runner::{backpack::BackPack, Robot, Runnable, Runner},
    utils::LibError,
    world::{
        coordinates::Coordinate,
        environmental_conditions::{EnvironmentalConditions, WeatherType},
        tile::{Content, Tile},
        World,
    },
};
//use searchtool_unwrap::{SearchDirection, SearchTool};
use sense_and_find_by_rustafariani::*;
use std::{
    borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, process::exit, rc::Rc
};

fn main() {
    let mut generator = MyWorldGen::new_param(200, 1, 5, 5, false, false, 2, false, None);
    let channel = Rc::new(RefCell::new(Channel::default()));
    let mut robot = MyRobot::new(channel);

    let gui_runner = GuiRunner::new(Box::new(robot), &mut generator).unwrap();

    gui_runner.run().unwrap();
}