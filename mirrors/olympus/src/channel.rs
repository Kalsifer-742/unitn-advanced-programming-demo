use std::collections::HashMap;

use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::Runnable;
use robotics_lib::world::environmental_conditions::{DayTime, EnvironmentalConditions, WeatherType};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;

#[allow(dead_code)]
enum MessageType {
    GameUpdate,
    WeatherUpdate
}

#[allow(dead_code)]
struct Message {
    message_type: MessageType,
    content: Vec<bool>,
}

pub(super) struct ChannelData {
    pub explored_world_map: Vec<Vec<Option<Tile>>>,
    pub robot_coordinates: (usize, usize),
    pub robot_energy: usize,
    pub robot_backpack_contents: HashMap<Content, usize>,
    pub robot_backpack_size: usize,
    pub discoverable_tiles: usize,
    pub robot_score: f32,
    pub time_of_day: DayTime,
    pub time_of_day_string: String,
    pub weather_condition: WeatherType,
}

impl Default for ChannelData {
    fn default() -> Self {
        Self {
            explored_world_map: vec![vec![None]],
            robot_coordinates: (0, 0),
            robot_energy: 0,
            robot_backpack_contents: HashMap::default(),
            robot_backpack_size: 0,
            discoverable_tiles: usize::MAX,
            robot_score: 0.0,
            time_of_day: DayTime::Morning,
            time_of_day_string: "00:00".to_string(),
            weather_condition: WeatherType::Sunny,
        }
    }
}

pub struct Channel {
    data: ChannelData,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl Channel {
    pub(super) fn receive(&self) -> &ChannelData {
        &self.data
    }

    pub fn send_game_info(&mut self, robot: & impl Runnable, world: &mut World) {
        self.data.explored_world_map = robot_map(world).expect("Problem calling robot_map (probably Mutex problems)");
        self.data.robot_coordinates = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
        self.data.robot_energy = robot.get_energy().get_energy_level();
        self.data.robot_backpack_contents = robot.get_backpack().get_contents().clone();
        self.data.robot_backpack_size = robot.get_backpack().get_size();
        self.data.discoverable_tiles = world.get_discoverable();
        self.data.robot_score = get_score(world);
    }

    pub fn send_weather_info(&mut self, weather: EnvironmentalConditions) {
        self.data.time_of_day = weather.get_time_of_day();
        self.data.time_of_day_string = weather.get_time_of_day_string();
        self.data.weather_condition = weather.get_weather_condition();
    }
}