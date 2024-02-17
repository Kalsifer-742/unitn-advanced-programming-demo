use std::process::Command;

use cursive::align::HAlign;
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, RadioGroup, SelectView, TextView};
use cursive::traits::*;
use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use midgard::world_visualizer::WorldVisualizer;
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
        .button("Confirm", |s| {})
    );
}

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::text("What do you want to do?")
        //.title("Question 1")
        .button("View World generator", move |s| start_midgard_visualizer(s))
        .button("Play game", |s| open_game_settigns(s))
    );

    siv.run();
}