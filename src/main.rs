use std::cell::RefCell;
use std::rc::Rc;
use cursive::Cursive;
use cursive::views::{Dialog, DummyView, LinearLayout, RadioGroup, TextView};
use midgard::world_generator::{self, WorldGenerator, WorldGeneratorParameters};
use midgard::world_visualizer::WorldVisualizer;
use olympus::channel::Channel;
use olympus::Visualizer;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::runner::Runnable;
use robotics_lib::world::world_generator::Generator;
use macroquad::prelude::*;

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

fn open_game_settigns(s: &mut Cursive) {
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
        .button("Confirm", |s| { 
                //start_game(s, robot_radio.selection(), world_generator_radio.selection(), visualizer_radio.selection());
            } 
        )
    );
}

async fn start_game(s: &mut Cursive, robot_selection: String, world_generator_selection: String, visualizer_selection: String) {
    s.quit();

    //TO-DO robot
    let world_generator = get_world_generator(world_generator_selection);

    if visualizer_selection == "oly" {
        let channel = Rc::new(RefCell::new(Channel::default()));
        let world_size = 200;
        //robots need to have a channel
        //let robot = DummyRobot::new(Rc::clone(&channel));
        //let mut visualizer = Visualizer::new(robot, world_generator, world_size, Rc::clone(&channel));
        //visualizer.start().await;
    } else if visualizer_selection == "rag" {
        //let visualizer = ragnarok::GuiRunner::new(robot, world_generator).expect("Error during ragnarok creation");
        //visualizer.run().expect("Error running ragnarok")
    }
}

fn get_world_generator(selection: String) -> Box<dyn Generator> {
    let world_size = 200;

    if selection == "rip" {
        Box::new(MyWorldGen::new_param(
    world_size,
            5,
            3,
            3,
            true,
            true,
            3,
            false,
            None
        ))
    } else {
        let params = WorldGeneratorParameters {
            world_size,
            amount_of_streets: Some(0.7),
            ..Default::default()
        };
        Box::new(WorldGenerator::new(params))
    }
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

//#[macroquad::main(window_conf)]
//async fn main() {
fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("What do you want to do?")
        .button("View World generator", move |s| start_midgard_visualizer(s))
        .button("Play game", |s| open_game_settigns(s))
    );

    siv.run();
}