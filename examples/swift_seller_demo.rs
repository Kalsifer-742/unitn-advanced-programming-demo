use std::collections::VecDeque;
// Energy and Event
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{destroy, go, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::Robot;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;
use ragnarok::GuiRunner;
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::world_generator::World;
use swift_seller::SwiftSeller;

pub struct DemoWorld;

impl DemoWorld {
    pub fn init() -> Self {
        Self {}
    }
}

/**************************************************************************
*  MAP:
*    ___________________________________________________
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    |   None     |   None     |   None     |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |  Shallow   |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    | Market(1)  |  Rock(1)   |  Fish(1)   |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    |   None     |   None     |  Rock(1)   |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    | Market(2)  |  Tree(1)   |  Tree(2)   |
*   |____________|____________|____________|____________|
*
*   Copyright: comment format courtesy of the common crate
*/

impl Generator for DemoWorld {
    fn gen(&mut self) -> World {

        // Default world

        let mut map: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..4 {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..4 {
                row.push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                })
            }
            map.push(row);
        }
        map[1][3] = Tile {
            tile_type: TileType::ShallowWater,
            content: Content::None,
            elevation: 0,
        };

        // Adding the contents

        map[1][1].content = Content::Market(1);
        map[1][2].content = Content::Rock(1);
        map[1][3].content = Content::Fish(1);
        map[2][3].content = Content::Rock(1);
        map[3][1].content = Content::Market(2);
        map[3][2].content = Content::Tree(1);
        map[3][3].content = Content::Tree(2);

        // On this demo world, the sun always shines
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap();

        // Return the world

        (map, (0, 0), environmental_conditions, 100.0, None)
    }
}

/// Create a 3x3 world where the robot spawns right next to a Market content tile
pub fn main() {
    struct MyRobot {
        robot: Robot,
        go_directions: VecDeque<Direction>,
        destroy_directions: VecDeque<Direction>,
        tool_params: VecDeque<Vec<Content>>,
        azioni: VecDeque<i32>,
    }

    impl MyRobot {
        pub fn new() -> Self {
            Self {
                robot: Robot::new(),
                go_directions: VecDeque::from([
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                ]),
                destroy_directions: VecDeque::from([
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    ]),
                tool_params: VecDeque::from([vec![Content::Fish(0), Content::Rock(0), Content::Tree(0)],vec![Content::Fish(0), Content::Rock(0), Content::Tree(0)], vec![Content::Tree(0)]]),
                azioni: VecDeque::from([0, 0, 0, 2, 0, 1, 1, 0, 2, 0, 1, 0, 1, 1, 0, 0, 2]),            
            }
        }
    }

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
            match self.azioni.pop_front() {
                None => {},
                Some(id) => {
                    match id {
                        0 => { 
                            let dir = self.go_directions.pop_front().unwrap();
                            let _ = go(self, world, dir); 
                        }
                        1 => { 
                            let dir = self.destroy_directions.pop_front().unwrap();
                            let _ = destroy(self, world, dir);
                        }
                        2 => { 
                            let vec = self.tool_params.pop_front().unwrap();
                            let _ = SwiftSeller::swift_seller(self, world, vec); 
                        }
                        _ => {},
                    }
                }                
            }
        }

        fn handle_event(&mut self, _event: Event) {
            // react to this event in a GUI
        }

        fn get_energy(&self) -> &Energy {
            &self.robot.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.robot.energy
        }

        fn get_coordinate(&self) -> &Coordinate {
            &self.robot.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.robot.coordinate
        }

        fn get_backpack(&self) -> &BackPack {
            &self.robot.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.robot.backpack
        }
    }

    // Instances
    let robot = MyRobot::new();
    let mut pwg = DemoWorld::init();

    let gui_runner = GuiRunner::new(Box::new(robot), &mut pwg).unwrap();

    gui_runner.run().unwrap();
}