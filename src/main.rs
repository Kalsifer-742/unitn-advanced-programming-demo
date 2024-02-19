use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use ai::MyRobot;
use bmo::BMO;
use cursive::Cursive;
use cursive::views::{Dialog, DummyView, LinearLayout, RadioGroup, TextView};
use midgard::params::WorldGeneratorParameters;
use midgard::{WorldGenerator, WorldVisualizer};
use olympus::channel::Channel;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::runner::Runnable;
use robotics_lib::world::world_generator::Generator;

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

fn start_swift_seller_demo(s: &mut Cursive) {
    s.quit();

    let command = "target/release/examples/swift_seller_demo";
    Command::new(command)
        .output()
        .expect("Error running swift_seller demo");
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
        .child(robot_radio.button("ai", "ai"))
        .child(robot_radio.button("bmo", "bmo"));
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
                start_game(
                    s,
                    robot_radio.selection().to_string(),
                    world_generator_radio.selection().to_string(),
                    visualizer_radio.selection().to_string()
                );
            } 
        )
    );
}

fn start_game(s: &mut Cursive, robot_selection: String, world_generator_selection: String, visualizer_selection: String) {
    s.quit();

    //robots need to have a channel for compatibility with olympus
    let channel = Rc::new(RefCell::new(Channel::default()));
    let robot = get_robot(robot_selection.clone(), Rc::clone(&channel));
    let (mut world_generator, _world_size) = get_world_generator(world_generator_selection.clone());

    let visualizer_selection = visualizer_selection.to_string();
    if visualizer_selection == "oly" {
        start_olympus_visualizer(robot_selection.clone(), world_generator_selection.clone());
    } else if visualizer_selection == "rag" {
        let visualizer = ragnarok::GuiRunner::new(robot, &mut  world_generator).expect("Error during ragnarok creation");
        visualizer.run().expect("Error running ragnarok")
    }
}

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

fn start_olympus_visualizer(robot_selection: String, world_generator_selection: String) {
    let command = "target/release/examples/olympus_demo";
    let args = [robot_selection, world_generator_selection];
    Command::new(command)
        .args(&args)
        .output()
        .expect("Error running olympus demo");
}

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("What do you want to do?")
        .button("Play game", |s| open_game_settings(s))
        .button("View midgard world generator", move |s| start_midgard_visualizer(s))
        .button("Try swift_seller tool", move |s| start_swift_seller_demo(s))
    );

    siv.run();
}