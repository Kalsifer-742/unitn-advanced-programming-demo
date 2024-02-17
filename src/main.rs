use std::cell::RefCell;
use std::rc::Rc;
use cursive::Cursive;
use cursive::views::{Dialog, DummyView, LinearLayout, RadioGroup, TextView};
use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use midgard::world_visualizer::WorldVisualizer;
use olympus::channel::Channel;
use olympus::Visualizer;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::world_generator::Generator;
use macroquad::prelude::*;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::Direction;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use strum::IntoEnumIterator;
struct PlaceholderRobot {
    robot: Robot,
}
impl Runnable for PlaceholderRobot {
    fn process_tick(&mut self, world: &mut World) {
        let directions : Vec<_> = Direction::iter().collect();
        let direction = directions[rand::gen_range(0, directions.len())].clone();
        let _ = robotics_lib::interface::go(self, world, direction);
    }
    fn handle_event(&mut self, _event: Event) {}
    fn get_energy(&self) -> &Energy { &self.robot.energy }
    fn get_energy_mut(&mut self) -> &mut Energy { &mut self.robot.energy }
    fn get_coordinate(&self) -> &Coordinate { &self.robot.coordinate }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate { &mut self.robot.coordinate }
    fn get_backpack(&self) -> &BackPack { &self.robot.backpack }
    fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.robot.backpack }
}
fn start_midgard_visualizer(s: &mut Cursive) {
    s.quit();

    WorldVisualizer::visualize_realtime(|| {
        let params = WorldGeneratorParameters {
            world_size: 500,
            amount_of_streets: Some(0.7),
            ..Default::default()
        };
        let mut world_generator = WorldGenerator::new(params);
        let (world, _spawn_point, _weather, _max_score, _score_table) = world_generator.gen();
        world
    }, 1000)
}

fn open_game_settings(s: &mut Cursive) {
    let mut robot_radio = RadioGroup::new();
    let mut world_generator_radio = RadioGroup::new();
    let mut visualizer_radio = RadioGroup::new();

    let robot_text = TextView::new("Robot");
    let world_generator_text = TextView::new("World Generator:");
    let visualizer_text = TextView::new("Visualizer:");

    let robot_layout = LinearLayout::vertical()
        .child(robot_text)
        .child(robot_radio.button("bmo", "bmo"))
        .child(robot_radio.button("andr", "androgeo"));
    let world_generator_layout = LinearLayout::vertical()
        .child(world_generator_text)
        .child(world_generator_radio.button("rip", "rip_worldgenerator"))
        .child(world_generator_radio.button("mid", "midgard"));
    let visualizer_layout = LinearLayout::vertical()
        .child(visualizer_text)
        .child(visualizer_radio.button("oly", "olympus"))
        .child(visualizer_radio.button("rag", "ragnarok"));

    s.pop_layer();
    s.add_layer(
        Dialog::new()
        .title("Game settings")
        .content(
            LinearLayout::horizontal()
            .child(robot_layout)
            .child(DummyView)
            .child(world_generator_layout)
            .child(DummyView)
            .child(visualizer_layout)
        )
        .button("Confirm", move |s| {
                start_game(s,
                    robot_radio.selection().to_string(),
                    world_generator_radio.selection().to_string(),
                    visualizer_radio.selection().to_string()
                );
            } 
        )
    );
}

//used for getting a impl Generator from a dyn Generator
struct GeneratorWrapper {
    generator: Box<dyn Generator>
}
impl Generator for GeneratorWrapper {
    fn gen(&mut self) -> robotics_lib::world::world_generator::World { self.generator.gen() }
}
impl GeneratorWrapper {
    pub fn new(generator: Box<dyn Generator>) -> Self { Self { generator } }
}

fn start_game(s: &mut Cursive, robot_selection: String, world_generator_selection: String, visualizer_selection: String) {
    s.quit();

    let mut world_generator = get_world_generator(world_generator_selection.to_string());

    //robots need to have a channel for compatibility with olympus
    let channel = Rc::new(RefCell::new(Channel::default()));
    let robot = get_robot(robot_selection, &channel);
    // let robot = PlaceholderRobot {robot: Robot::new()};

    let visualizer_selection = visualizer_selection.to_string();
    if visualizer_selection == "oly" {
        // let world_size = 200;
        // let mut visualizer = Visualizer::new(robot, world_generator, world_size, Rc::clone(&channel));
        // visualizer.start().await;
    } else if visualizer_selection == "rag" {
        let visualizer = ragnarok::GuiRunner::new(robot, &mut  world_generator).expect("Error during ragnarok creation");
        visualizer.run().expect("Error running ragnarok")
    }
}

fn get_world_generator(selection: String) -> GeneratorWrapper {
    let world_size = 200;

    let generator = if selection == "rip" {
        Box::new(
            MyWorldGen::new_param(
                world_size, 5, 3, 3,
                true, true, 3, false, None
            )
        ) as Box::<dyn Generator>
    } else {
        let params = WorldGeneratorParameters {
            world_size,
            amount_of_streets: Some(0.7),
            ..Default::default()
        };
        Box::new(WorldGenerator::new(params))
        as Box::<dyn Generator>
    };
    return GeneratorWrapper::new(generator);
}

fn get_robot(_selection: String, _channel: &Rc<RefCell<Channel>>) -> Box<dyn Runnable> {
    Box::new(PlaceholderRobot { robot: Robot::new() })
}

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Olympus".to_string(),
//         window_width: 1920,
//         window_height: 1080,
//         fullscreen: false,
//         ..Default::default()
//     }
// }

// #[macroquad::main(window_conf)]
// async fn main() {
fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("What do you want to do?")
        .button("View World generator", move |s| start_midgard_visualizer(s))
        .button("Play game", |s| open_game_settings(s))
    );

    siv.run();
}