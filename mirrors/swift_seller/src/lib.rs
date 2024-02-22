use std::collections::HashMap;
use robotics_lib::interface::{Tools, robot_view, put, Direction};
use robotics_lib::world::World;
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;

pub struct SwiftSeller;
impl Tools for SwiftSeller {}

impl SwiftSeller {
    /// Auto-sell to a Market only the items you choose
    ///
    /// # Usage
    /// ```rust
    /// use swift_seller::SwiftSeller;
    /// ```
    ///
    /// # Arguments
    /// - `robot`: The robot
    /// - `world`: The world in which the robot is
    /// - `vec`: The items you want the tool to auto-sell
    ///
    /// # Returns
    /// - `HashMap<Content, usize>`: The items sold at the Market and their quantity
    /// - `LibError`: The error that occurred
    ///
    /// # Errors
    /// - `OperationNotAllowed`: The robot is not near a tile with a Market on it or has 0 interactions left to begin with
    /// - `NotEnoughSpace`: The robot doesn't have enough space for earned coins
    ///
    /// # Notes
    /// - if the market's interaction get to 0 between the sale of multiple valid items, the map containing the items sold up to that point is returned
    /// - does not support multiple robots
    pub fn swift_seller(
        robot: &mut impl Runnable,
        world: &mut World,
        vec: Vec<Content>
    ) -> Result<HashMap<Content, usize>, LibError> {

        // First of all, let's check if the robot happens to be near a tile with a Market on it

        let mut market_near: bool = false;
        let mut market_dir = Direction::Left; // initialized
        let mut interactions_left: usize = 0;
        let mut highest_interactions: usize = 0;

        let robot_view = robot_view(robot, &world);
        for (i, row) in robot_view.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match (i, j) {
                    (0, 1) | (1, 0) | (1, 2) | (2, 1) => {
                        match col {
                            | None => (),
                            | Some(tile) => {
                                match tile.content {
                                    Content::Market(n) => {
                                        if n > highest_interactions {
                                            interactions_left = n;
                                            highest_interactions = n;

                                            market_near = true;
                                            match (i, j) {
                                                (0, 1) => market_dir = Direction::Up,
                                                (1, 0) => market_dir = Direction::Left,
                                                (1, 2) => market_dir = Direction::Right,
                                                (2, 1) => market_dir = Direction::Down,
                                                _ => return Err(LibError::OperationNotAllowed)
                                            }

                                        }
                                    },
                                    _ => ()
                                }
                            }
                        }
                    },
                    _ => ()
                }
            }
        }

        // If the robot is NOT near a tile with a Market on it this tool cannot be used
        if !market_near {
            return Err(LibError::OperationNotAllowed);
        }
        // Straight off the bat, if the market has no interactions left, quit
        if interactions_left < 1 {
            return Err(LibError::OperationNotAllowed);
        }

        // If the robot is near a Market, sell the items held in its backpack which can be sold

        let mut _coins_earned: usize = 0;

        let mut items_sold: HashMap<Content, usize> = HashMap::new();

        let cloned_contents = robot.get_backpack().get_contents().clone();

        // If the tool sells at least one item, return it when the interactions left get to 0
        let mut sold_anything:bool = false;

        // Sell items in order given by the user
        for items in vec {
            // Allow selling only the items that can actually be sold
            match items {
                Content::Rock(_) | Content::Fish(_) | Content::Tree(_) => {
                    for (item, qty) in cloned_contents.clone() {
                        if interactions_left < 1 {
                            return if sold_anything {
                                Ok(items_sold)
                            } else {
                                Err(LibError::OperationNotAllowed)
                            }
                        }
                        if items == item && qty > 0 {
                            match put(
                                robot,
                                world,
                                item.clone(),
                                qty,
                                market_dir.clone()
                            ) {
                                Ok(earned) => {
                                    _coins_earned += earned;
                                    let sold = qty - robot.get_backpack().get_contents().clone().get(&item).unwrap();
                                    items_sold.insert(item, sold);
                                    interactions_left -= 1;
                                    sold_anything = true;
                                },
                                Err(LibError::NotEnoughSpace(tried)) => {
                                    return Err(LibError::NotEnoughSpace(tried));
                                },
                                Err(e) => {
                                    eprintln!("ERR: {:?} - PUT arguments: {:?} {:?} {:?}", e, item.clone(), qty, market_dir.clone());
                                    panic!("UNEXPECTED ERROR - CONTACT THE GROUP")
                                }
                            }
                        }
                    }
                }
                _ => ()
            }
        }
        Ok(items_sold)
    }
}

#[cfg(test)]
mod tests {
    use std::process::exit;

    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;

    use robotics_lib::interface::{Direction, go, where_am_i, destroy };

    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runner};

    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
    use robotics_lib::world::tile::{Tile, TileType};
    use robotics_lib::world::world_generator::Generator;

    use super::*;

    /**************************************************************************
    *  MAP:
    *    ______________________________________
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |    0 el    |
    *   |    None    |  Tree(3)   |   Tree(4)  |
    *   |____________|____________|____________|
    *   |            |            |            |
    *   |   Shallow  |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |    0 el    |
    *   |   Fish(3)  | Market(1)  |   Rock(3)  |
    *   |____________|____________|____________|
    *   |            |            |            |
    *   |    Grass   |    Grass   |   Grass    |
    *   |    0 el    |    0 el    |    0 el    |
    *   |   Rock(2)  |    None    |   Rock(6)  |
    *   |____________|____________|____________|
    *
    *   Copyright: comment format courtesy of the common crate
    */

    fn test_world(market_interactions: usize) -> impl Generator {
        // World generator

        struct MarketWorld {
            market_interactions: usize
        }
        impl MarketWorld {
            fn new(market_interactions: usize) -> Self {
                MarketWorld {
                    market_interactions
                }
            }
        }
        impl Generator for MarketWorld {
            fn gen(&mut self) -> robotics_lib::world::world_generator::World {
                let mut map: Vec<Vec<Tile>> = Vec::new();

                map.push(Vec::new());
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(3),
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(4),
                    elevation: 0,
                });

                map.push(Vec::new());
                map[1].push(Tile {
                    tile_type: TileType::ShallowWater,
                    content: Content::Fish(3),
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Market(self.market_interactions),
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Rock(3),
                    elevation: 0,
                });

                map.push(Vec::new());
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Rock(2),
                    elevation: 0,
                });
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Rock(6),
                    elevation: 0,
                });

                let environmental_conditions =
                    EnvironmentalConditions::new(&[WeatherType::Sunny],
                                                 15,
                                                 12);
                (map, (0, 0), environmental_conditions.unwrap(), 100.0, None)
            }
        }

        MarketWorld::new(market_interactions)
    }

    fn start(run: Result<Runner, LibError>) {
        match run {
            | Ok(mut r) => {
                let _ = r.game_tick();
            }
            | Err(e) => {
                println!("{:?}", e);
                exit(1)
            },
        }
    }

    #[test]
    fn no_market_nearby() {

        // Generate the test world
        let mut generator = test_world(0);

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {

                // List all the movements I intend to make and the outcomes of the function call
                // after the corresponding movement
                let movements:&[Direction] = &[
                    Direction::Right, Direction::Right,
                    Direction::Down, Direction::Down,
                    Direction::Left, Direction::Left,
                    Direction::Up, Direction::Up
                ];

                // For each movement, perform the following actions
                for movement in movements {
                    // Since I created a world ad hoc, those movements should be possible
                    go(self, world, movement.clone()).expect("CANNOT MOVE");

                    // Every move I should NOT find a market nearby (or find one with value 0)
                    assert_eq!(
                        SwiftSeller::swift_seller(self, world, vec![Content::Fish(0), Content::Tree(0), Content::Rock(0)]),
                        Err(LibError::OperationNotAllowed)
                    )
                }
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        // Since the weather is sunny day, the robot is walking on grass, and it starts with full
        // energy, I can walk around the Market content all in one tick
        start(run);
    }

    #[test]
    fn sell_empty_backpack() {

        // Generate the test world
        let mut generator = test_world(10);

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {

                // List all the movements I intend to make and the outcomes of the function call
                // after the corresponding movement
                let movements:&[Direction] = &[
                    Direction::Right, Direction::Right,
                    Direction::Down, Direction::Down,
                    Direction::Left, Direction::Left,
                    Direction::Up, Direction::Up
                ];

                // For each movement, perform the following actions
                for (i, movement) in movements.iter().enumerate() {
                    // Since I created a world ad hoc, those movements should be possible
                    go(self, world, movement.clone()).expect("CANNOT MOVE");

                    // When I'm nearby a market, I should try to sell everything; I have an empty
                    // backpack, so the returned value from the function call should be:
                    let empty_map:HashMap<Content, usize> = HashMap::new();

                    // Let's check
                    if i % 2 == 0 {

                        assert_eq!(

                            SwiftSeller::swift_seller(self, world, vec![Content::Fish(0), Content::Tree(0), Content::Rock(0)]),
                            Ok(empty_map)
                        )
                    }
                }
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        // Since the weather is sunny day, the robot is walking on grass, and it starts with full
        // energy, I can walk around the Market content all in one tick
        start(run);
    }

    #[test]
    fn destroy_and_sell_items() {

        // Generate the test world
        let mut generator = test_world(10);

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {

                // List all the movements I intend to make and the outcomes of the function call
                // after the corresponding movement
                let movements:&[Direction] = &[
                    Direction::Right, Direction::Right,
                    Direction::Down, Direction::Down,
                    Direction::Left, Direction::Left,
                    Direction::Up, Direction::Up
                ];

                // For each movement, perform the following actions
                for  movement in movements {

                    println!("--------");

                    println!("Destroy {:?}", movement);
                    // Since I created a world ad hoc, these destroy() should have enough energy
                    match destroy(self, world, movement.clone()) {
                        Ok(_) => (),
                        Err(error) =>
                            eprintln!("ERR\n\tERROR: {:?}\n\tMOVEMENT: {:?}\n\tBACKPACK SIZE: {}",
                                      error,
                                      movement,
                                      self.get_backpack().get_size()
                            )
                    }

                    // Since I created a world ad hoc, those movements should be possible
                    println!("Go {:?}", movement);
                    go(self, world, movement.clone()).expect("CANNOT MOVE");

                    let (robot_view, _) = where_am_i(self, &world);

                    for row in robot_view.iter() {
                        for col in row.iter() {
                            match col {
                                | None => print!(" None "),
                                | Some(tile) => {
                                    if tile.content != Content::None {
                                        print!(" {:?}({:?}) ", tile.tile_type, tile.content)
                                    }
                                    else {
                                        print!(" {:?} ", tile.tile_type)
                                    }
                                },
                            }
                        }
                        println!();
                    }

                    // Now I can finally call the function to interact with the Market
                    println!("Sell?");
                    match SwiftSeller::swift_seller(self, world, vec![Content::Fish(0), Content::Tree(0), Content::Rock(0)]) {
                        Err(LibError::OperationNotAllowed) =>
                            eprintln!("No Market nearby!"),
                        Err(LibError::NotEnoughSpace(tried)) =>
                            eprintln!("Can't hold {} coins!", tried),
                        Err(any) =>
                            eprintln!("{:?}", any),
                        Ok(map) => {
                            println!("Sold to market:");
                            for (key, value) in map {
                                println!("\t- item: {}, qty: {}", key, value)
                            }
                        }
                    }
                }
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        // Since the weather is sunny day, the robot is walking on grass, and it starts with full
        // energy, I can walk around the Market content all in one tick
        start(run);
    }

    #[test]
    fn zero_energy() {
    // Generate the test world
        let mut generator = test_world(100);

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {
                println!("{:?}", self.get_energy());
                match destroy(self, world, Direction::Right) {
                    Ok(_) => (),
                    Err(error) =>
                        eprintln!("ERR\n\tERROR: {:?}\n\tMOVEMENT: {:?}\n\tBACKPACK SIZE: {}",
                                  error,
                                  Direction::Right,
                                  self.get_backpack().get_size()
                        )
                }
                println!("{:?}", self.get_energy());
                go(self, world, Direction::Right).expect("CANNOT MOVE");
                println!("{:?}", self.get_energy());
                for _ in 0..498 {
                    go(self, world, Direction::Right).expect("CANNOT MOVE");
                    go(self, world, Direction::Left).expect("CANNOT MOVE");
                }
                println!("{:?}", self.get_energy());
                println!("{:?}", self.get_backpack());

                match SwiftSeller::swift_seller(self, world, vec![Content::Tree(0)]) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                println!("{:?}", self.get_energy());
                println!("{:?}", self.get_backpack());
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        start(run);
    }

    // After selling some of the items, if the market gets to zero interactions the tool returns
    // an error, while it should return an empty hashmap/the map of items sold up to that point
    #[test]
    fn errore_alessandra() {
        // Generate the test world
        let mut generator = test_world(1);

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {
                // Destroy right to get trees
                let _ = destroy(self, world, Direction::Right);

                // Move right-right to get near rocks
                let _ = go(self, world, Direction::Right);
                let _ = go(self, world, Direction::Right);

                // Destroy down to get rocks
                let _ = destroy(self, world, Direction::Down);

                // Move down to get near rocks
                let _ = go(self, world, Direction::Down);

                // Start selling what you have

                let mut rock_map:HashMap<Content, usize> = HashMap::new();
                rock_map.insert(Content::Rock(0), 3);

                assert_eq!(SwiftSeller::swift_seller(
                    self,
                    world,
                    vec![Content::Rock(0), Content::Tree(0)]),
                    Ok(rock_map)
                );

            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        start(run);
    }

    /**************************************************************************
    *  TWO MARKETS WORLD (SPAWN at 1,1):
    *    ______________________________________
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |   0 el     |
    *   |    None    |  Tree(2)   |   None     |
    *    ______________________________________
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |   0 el     |
    *   |  Market(2) |   None     | Market(0)  |
    *    ______________________________________
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |   0 el     |
    *   |    None    |   None     |   None     |
    *   |____________|____________|____________|
    *
    *   Copyright: comment format courtesy of the common crate
    */

    // Spawns at (1,1) genera
    fn test_two_markets_world() -> impl Generator {
        // World generator

        struct TwoMarketsWorld { }
        impl TwoMarketsWorld {
            fn new() -> Self {
                TwoMarketsWorld { }
            }
        }
        impl Generator for TwoMarketsWorld {
            fn gen(&mut self) -> robotics_lib::world::world_generator::World {
                let mut map: Vec<Vec<Tile>> = Vec::new();

                map.push(Vec::new());
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(2),
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });

                map.push(Vec::new());
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Market(2),
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Market(0),
                    elevation: 0,
                });

                map.push(Vec::new());
                for _ in 0..3 {
                    map[2].push(Tile {
                        tile_type: TileType::Grass,
                        content: Content::None,
                        elevation: 0,
                    });
                }

                let environmental_conditions =
                    EnvironmentalConditions::new(&[WeatherType::Sunny],
                                                 15,
                                                 12);
                (map, (1, 1), environmental_conditions.unwrap(), 100.0, None)
            }
        }

        TwoMarketsWorld::new()
    }

    #[test]
    fn one_empty_market() {

        // Generate the test world
        let mut generator = test_two_markets_world();

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {

                // Destroy the tree on top
                match destroy(self, world, Direction::Up) {
                    Ok(_) => (),
                    Err(_) => eprintln!("Error when destroying")
                }

                match SwiftSeller::swift_seller(self, world, vec![Content::Tree(0)]) {
                    Err(LibError::OperationNotAllowed) =>
                        eprintln!("No Market nearby!"),
                    Err(LibError::NotEnoughSpace(tried)) =>
                        eprintln!("Can't hold {} coins!", tried),
                    Err(any) =>
                        eprintln!("{:?}", any),
                    Ok(map) => {
                        println!("Sold to market:");
                        for (key, value) in map {
                            println!("\t- item: {}, qty: {}", key, value)
                        }
                    }
                }
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        start(run);
    }
}
