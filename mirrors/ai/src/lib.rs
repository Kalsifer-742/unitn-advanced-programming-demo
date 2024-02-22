use asfalt_inator::AsfaltInator;
use cargo_commandos_lucky::lucky_function::lucky_spin;
use olympus::channel::Channel;
use robotics_lib::{
    energy::Energy,
    event::events::Event,
    interface::{
        destroy, discover_tiles, go, look_at_sky, robot_map, robot_view, Direction,
    },
    runner::{backpack::BackPack, Robot, Runnable},
    utils::{calculate_cost_go_with_environment, LibError},
    world::{
        coordinates::Coordinate,
        environmental_conditions::EnvironmentalConditions,
        tile::{Content, Tile},
        World,
    },
};
use sense_and_find_by_rustafariani::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    vec,
};

pub struct MyRobot {
    pub robot: Robot,
    pub ticks: i32,
    channel: Rc<RefCell<Channel>>,
}

impl Runnable for MyRobot {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::TimeChanged(weather) => {
                self.channel.borrow_mut().send_weather_info(weather);
            }
            _ => {}
        }
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

    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.channel.borrow_mut().send_game_info(self, world);

        // Variables to base decisions on
        let variables: Variables = Variables::new(
            self.robot.energy.get_energy_level(),
            self.robot.backpack.get_contents().clone(),
            robot_map(world).unwrap(),
            look_at_sky(world),
            self.ticks,
        );
        robot_view(self, world);
        // Decision process
        let complex_actions = variables.interpreter();
        for action in complex_actions {
            match action {
                // Uses sense_raw_centered_square to discover some tiles in the world
                ComplexAction::Discover => {
                    let mut lssf = Lssf::new();
                    let res: Result<Vec<Vec<((usize, usize), Tile, bool)>>, LibError> =
                        lssf.sense_raw_centered_square(41, world, self, 2);
                }
                ComplexAction::AsfaltInator => {}
                // Using sense_and_find make decision on how to 
                // explore the world based on partial info
                ComplexAction::Explore => {
                    let c = self.get_tile_to_move_towards(world, 21, 4);
                    let mut flag = false;
                    let mut coords = (
                        self.robot.coordinate.get_row(),
                        self.robot.coordinate.get_col(),
                    );
                    for dir in c.iter() {
                        match dir {
                            Direction::Up => coords = (coords.0 - 1, coords.1),
                            Direction::Right => coords = (coords.0, coords.1 + 1),
                            Direction::Down => coords = (coords.0 + 1, coords.1),
                            Direction::Left => coords = (coords.0, coords.1 - 1),
                        }
                        if flag {
                            let res = discover_tiles(self, world, &[(coords.0, coords.1)]);
                        } else {
                            let res = go(self, world, dir.clone());
                            if res.is_err() {
                                flag = true;
                            }
                            robot_view(self, world);
                            for d in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
                                let _ = destroy(self, world, d);
                            }
                        }
                    }
                }
                ComplexAction::GetResources => {}
                // Wait and try to replenish energy with lucky_spin
                ComplexAction::Wait => {
                    let _ = lucky_spin(&mut self.robot);
                }
            }
        }
        self.ticks += 1;
    }
}

impl MyRobot {
    pub fn new(channel: Rc<RefCell<Channel>>) -> Self {
        Self {
            robot: Robot::new(),
            ticks: 0,
            channel,
        }
    }
    // Function that decides where to go in Explore action
    pub fn get_tile_to_move_towards(
        &mut self,
        world: &mut World,
        l: usize,
        granularity: usize,
    ) -> Vec<Direction> {
        let robot_c = (
            self.robot.coordinate.get_row(),
            self.robot.coordinate.get_col(),
        );
        let coords = check_coords(robot_c, l);
        let mut lssf = Lssf::new();
        let mut map: Vec<Vec<((usize, usize), Tile, bool)>> = lssf
            .sense_raw_square_by_center(l, world, self, granularity, coords)
            .unwrap();

        // Create a map to make decisions on
        let world_map = robot_map(world).unwrap();
        for row in map.iter_mut() {
            for col in row {
                match col {
                    ((r, c), _, false) => {
                        if world_map[*r][*c].is_some() {
                            col.1.tile_type = world_map[*r][*c].as_ref().unwrap().tile_type;
                            col.2 = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut matrix_likeability: Vec<Vec<(i32, Vec<Direction>)>> =
            vec![vec![(0, vec![]); map.len()]; map.len()];
        let mut matrix_visited: Vec<Vec<bool>> = vec![vec![false; map.len()]; map.len()];
        let rel_robot_c_row = (robot_c.0 as i32 - coords.0 as i32 + 10) as usize;
        let rel_robot_c_col = (robot_c.1 as i32 - coords.1 as i32 + 10) as usize;
        path_finder(
            (rel_robot_c_row, rel_robot_c_col),
            None,
            &map,
            &mut matrix_likeability,
            &mut matrix_visited,
            &mut vec![],
            0,
            look_at_sky(world),
            1,
        );
        let mut path: Vec<Direction> = vec![];
        let mut max: i32 = 0;
        for row in matrix_likeability {
            for col in row {
                if col.0 > max {
                    path = col.1.clone();
                    max = col.0;
                }
                //print!("{:?} ", col.0);
            }
            //println!();
        }
        //println!("{:?}", max);
        path
    }
}
// Function to transform relative coordinates to global
pub fn check_coords(robot_c: (usize, usize), l: usize) -> (usize, usize) {
    let mut robot_c = robot_c;
    if robot_c.0 as i32 - l as i32 / 2 < 0 {
        robot_c.0 = robot_c.0 + (robot_c.0 as i32 - l as i32 / 2).abs() as usize;
    }
    if robot_c.1 as i32 - l as i32 / 2 < 0 {
        robot_c.1 = robot_c.1 + (robot_c.1 as i32 - l as i32 / 2).abs() as usize;
    }
    if robot_c.0 as i32 + l as i32 / 2 > 199 {
        robot_c.0 = robot_c.0 - ((robot_c.0 + l / 2) - 199);
    }
    if robot_c.1 as i32 + l as i32 / 2 > 199 {
        robot_c.1 = robot_c.1 - ((robot_c.1 + l / 2) - 199);
    }
    robot_c
}
// path_finder
pub fn path_finder(
    curr: (usize, usize),
    prev: Option<(usize, usize)>,
    map: &Vec<Vec<((usize, usize), Tile, bool)>>,
    matrix_likeability: &mut Vec<Vec<(i32, Vec<Direction>)>>,
    matrix_visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<Direction>,
    cost: usize,
    environmental_conditions: EnvironmentalConditions,
    uncovered_tiles: usize,
) {
    matrix_visited[curr.0][curr.1] = true;
    let dirs = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    if map[curr.0][curr.1].1.tile_type.properties().walk() {
        for dir in dirs {
            let mut base_cost;
            let next: Option<(usize, usize)> = match dir {
                Direction::Up => {
                    if curr.0 as i32 - 1 < 0
                        || matrix_visited[curr.0 - 1][curr.1]
                        || !map[curr.0 - 1][curr.1].1.tile_type.properties().walk()
                    {
                        None
                    } else {
                        Some((curr.0 - 1, curr.1))
                    }
                }
                Direction::Right => {
                    if curr.1 + 1 > map.len() - 1
                        || matrix_visited[curr.0][curr.1 + 1]
                        || !map[curr.0][curr.1 + 1].1.tile_type.properties().walk()
                    {
                        None
                    } else {
                        Some((curr.0, curr.1 + 1))
                    }
                }
                Direction::Down => {
                    if curr.0 + 1 > map.len() - 1
                        || matrix_visited[curr.0 + 1][curr.1]
                        || !map[curr.0 + 1][curr.1].1.tile_type.properties().walk()
                    {
                        None
                    } else {
                        Some((curr.0 + 1, curr.1))
                    }
                }
                Direction::Left => {
                    if curr.1 as i32 - 1 < 0
                        || matrix_visited[curr.0][curr.1 - 1]
                        || !map[curr.0][curr.1 - 1].1.tile_type.properties().walk()
                    {
                        None
                    } else {
                        Some((curr.0, curr.1 - 1))
                    }
                }
            };
            if prev.is_some() {
                base_cost = map[curr.0][curr.1].1.tile_type.properties().cost();
                base_cost = calculate_cost_go_with_environment(
                    base_cost,
                    environmental_conditions.clone(),
                    map[curr.0][curr.1].1.tile_type,
                );
                let cost = cost + base_cost;
                if cost < 200 {
                    let mut u_tile = uncovered_tiles;
                    if map[curr.0][curr.1].2 == false {
                        u_tile += 10;
                    }

                    let mut distance_row = curr.0 as i32 - (map.len() as i32 / 2);
                    if distance_row < 0 {
                        distance_row = distance_row * -1
                    }
                    let mut distance_col = curr.1 as i32 - (map.len() as i32 / 2);
                    if distance_col < 0 {
                        distance_col = distance_col * -1
                    }
                    let distance = distance_row + distance_col;
                    let likeability = u_tile as i32 * distance;
                    if likeability >= matrix_likeability[curr.0][curr.1].0 {
                        matrix_likeability[curr.0][curr.1] = (likeability, path.clone());
                        if next.is_some()
                            && map[next.unwrap().0][next.unwrap().1]
                                .1
                                .tile_type
                                .properties()
                                .walk()
                        {
                            path.push(dir);
                            path_finder(
                                next.unwrap(),
                                Some(curr),
                                map,
                                matrix_likeability,
                                matrix_visited,
                                path,
                                cost + 20,
                                environmental_conditions.clone(),
                                u_tile,
                            );
                            path.pop();
                        }
                    }
                }
            } else {
                if next.is_some()
                    && map[next.unwrap().0][next.unwrap().1]
                        .1
                        .tile_type
                        .properties()
                        .walk()
                {
                    path.push(dir);
                    path_finder(
                        next.unwrap(),
                        Some(curr),
                        map,
                        matrix_likeability,
                        matrix_visited,
                        path,
                        cost + 5,
                        environmental_conditions.clone(),
                        uncovered_tiles,
                    );
                    path.pop();
                }
            }
        }
    }
    matrix_visited[curr.0][curr.1] = false;
}

pub enum ComplexAction {
    Discover,
    Explore,
    GetResources,
    AsfaltInator,
    Wait,
}
pub struct Variables {
    energy_lv: usize,
    inventory: HashMap<Content, usize>,
    map: Vec<Vec<Option<Tile>>>,
    e: EnvironmentalConditions,
    ticks: i32,
    city1: Option<(usize, usize)>,
    city2: Option<(usize, usize)>,
}

impl Variables {
    fn new(
        energy_lv: usize,
        inventory: HashMap<Content, usize>,
        map: Vec<Vec<Option<Tile>>>,
        e: EnvironmentalConditions,
        ticks: i32,
    ) -> Self {
        Self {
            energy_lv,
            inventory,
            map,
            e,
            ticks,
            city1: None,
            city2: None,
        }
    }
    fn interpreter(&self) -> Vec<ComplexAction> {
        let mut action: Vec<ComplexAction> = Vec::new();
        let mut flag = true;
        let mut cycles = 0;
        while flag {
            if self.ticks == 0 && cycles == 0 {
                action.push(ComplexAction::Discover);
                flag = false;
            } else if self.energy_lv < 200 {
                action.push(ComplexAction::Wait);
                flag = false;
            } else {
                action.push(ComplexAction::Explore);
                flag = false;
            }
            cycles += 1;
        }
        action
    }
}