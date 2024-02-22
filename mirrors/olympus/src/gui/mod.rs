use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use custom_camera::CustomCamera;
use renderer::Renderer;
use ui::UI;

use crate::channel::ChannelData;
use renderer::RendererProps;
use ui::UIProps;

use self::keyboard_controls::KeyboardControls;

mod keyboard_controls;
mod custom_camera;
mod renderer;
mod ui;

pub(super) struct GUI {
    camera: CustomCamera,
    renderer: Renderer,
    pub(super) ui: UI,
    keyboard_controls: KeyboardControls,
    show_hud: bool,
}

impl GUI {
    pub(super) fn new(world_size: usize, tick_time: Rc<RefCell<f32>>) -> Self {
        Self {
            camera: Default::default(),
            renderer: Renderer::new(world_size),
            ui: UI::new(tick_time),
            keyboard_controls: Default::default(),
            show_hud: true,
        }
    }

    pub(super) fn handle_input(&mut self) {
        if is_key_pressed(self.keyboard_controls.toggle_hud) {
            self.show_hud = !self.show_hud;
        }

        self.camera.handle_input();
        self.ui.handle_input();
    }
    
    fn update_camera(&mut self) {
        self.camera.update();
        set_camera(self.camera.get_actual_camera());
    }
    
    fn render_game(&self, data: &ChannelData, camera_front: Vec3) {
        self.renderer.render(
            RendererProps { 
                explored_world_map: &data.explored_world_map,
                robot_coordinates: data.robot_coordinates,
                time_of_day: data.time_of_day
            },
            self.ui.is_day_light_cycle_on(),
            camera_front
        );
    }
    
    fn render_ui(&mut self, data: &ChannelData) {
        set_default_camera();
        self.ui.render(
            UIProps { 
                explored_world_map: &data.explored_world_map,
                robot_coordinates: data.robot_coordinates,
                robot_energy: data.robot_energy,
                robot_backpack_contents: &data.robot_backpack_contents,
                robot_backpack_size: data.robot_backpack_size,
                discoverable_tiles: data.discoverable_tiles,
                robot_score: data.robot_score,
                time_of_day: data.time_of_day,
                time_of_day_string: data.time_of_day_string.clone(),
                weather_condition: data.weather_condition
            }
        );
    }

    pub(super) fn render(&mut self, data: &ChannelData) {
        if self.ui.is_mouse_grabbed() {
            self.update_camera(); // This needs to be done first
        } else {
            set_camera(self.camera.get_actual_camera());
        }
        
        self.render_game(data, self.camera.get_front());
        if self.show_hud {
            self.render_ui(data);
        }
    }
}