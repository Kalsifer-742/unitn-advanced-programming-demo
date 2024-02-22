use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use macroquad::prelude::*;
use macroquad::telemetry::textures_count;
use macroquad::ui::{root_ui, widgets, Layout};
use macroquad::hash;
use robotics_lib::world::environmental_conditions::{DayTime, WeatherType};
use robotics_lib::world::tile::{Content, Tile};
use sys_info::{cpu_num, cpu_speed, mem_info, os_release, MemInfo};
use crate::gui::keyboard_controls::KeyboardControls;

pub(crate) struct UI {
    viewport_width: f32,
    viewport_height: f32,
    keyboard_controls: KeyboardControls,
    show_tile_info: bool,
    show_help: bool,
    show_stats: bool,
    quit_requested: bool,
    exit: bool,
    is_mouse_grabbed: bool,
    mouse_grabbed_flag: bool,
    old_grab_status: bool,
    tick_time: Rc<RefCell<f32>>,
    daylight_cycle: bool,
}

pub(super) struct UIProps<'a> {
    pub explored_world_map: &'a Vec<Vec<Option<Tile>>>,
    pub discoverable_tiles: usize,
    pub robot_coordinates: (usize, usize),
    pub robot_energy: usize,
    pub robot_backpack_contents: &'a HashMap<Content, usize>,
    pub robot_backpack_size: usize,
    pub robot_score: f32,
    pub time_of_day: DayTime,
    pub time_of_day_string: String,
    pub weather_condition: WeatherType,
}

impl UI {
    pub(super) fn new(tick_time: Rc<RefCell<f32>>) -> Self {
        Self {
            viewport_width: screen_width(),
            viewport_height: screen_height(),
            keyboard_controls: Default::default(),
            show_tile_info: false,
            show_help: false,
            show_stats: false,
            quit_requested: false,
            exit: false,
            is_mouse_grabbed: false,
            mouse_grabbed_flag: true,
            old_grab_status: false,
            tick_time,
            daylight_cycle: true
        }
    }

    pub(super) fn is_mouse_grabbed(&self) -> bool {
        self.is_mouse_grabbed
    }

    fn grab_mouse(&self, grab: bool) {
        if grab {
            set_cursor_grab(true);
            show_mouse(false);
        } else {
            set_cursor_grab(false);
            show_mouse(true);
        }
    }

    pub(crate) fn toggle_mouse_grab(&mut self) {
        self.is_mouse_grabbed = !self.is_mouse_grabbed;
        self.grab_mouse(self.is_mouse_grabbed);
    }

    pub(super) fn handle_input(&mut self) {
        if is_key_pressed(self.keyboard_controls.toggle_tile_info) {
            self.show_tile_info = !self.show_tile_info;
        }
        if is_key_pressed(self.keyboard_controls.toggle_free_mouse) {
            self.toggle_mouse_grab();
        }
        if is_key_pressed(self.keyboard_controls.toggle_help) {
            self.show_help = !self.show_help;
        }
        if is_key_pressed(self.keyboard_controls.toggle_statistics) {
            self.show_stats = !self.show_stats;
        }
        // if is_key_pressed(self.keyboard_controls.take_screenshot) {
        //     //set_default_camera();
        //     get_screen_data().export_png("screenshots/screenshot.png");
        // }
        if is_key_pressed(self.keyboard_controls.exit){
            self.quit_requested = true;
        }
    }

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }

    fn show_game_info(&mut self, props: &UIProps) {
        let position = vec2(self.viewport_width - 400.0, 0.0);
        let size = vec2(400.0, 700.0);
        
        widgets::Window::new(
            hash!("game_info_window"),
            position,
            size
        )
        .label("Robot")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, format!("Game tick interval: ").as_str());
            ui.slider(hash!("tick_time_slider"), "[0.0 - 5.0]", 0.0..5.0, &mut self.tick_time.borrow_mut());
            ui.checkbox(hash!("daylight_cicle_checkbox"), "Show daylight cycle", &mut self.daylight_cycle);
            ui.label(None, "Energy: ");
            let max_energy_level = 1000.0; //const MAX_ENERGY_LEVEL: usize = 1000;
            let cursor = ui.canvas().cursor();
            ui.canvas().rect(
                Rect::new(
                    cursor.x,
                    cursor.y,
                    Self::map_range(props.robot_energy as f32, 0.0, max_energy_level, 1.0, 300.0),
                    20.0
                ),
                BLACK,
                YELLOW,
            );
            ui.separator();

            ui.label(None, &format!("Coordinates X: {}, Y: {}", props.robot_coordinates.0, props.robot_coordinates.1));
            ui.label(None, &format!("Backpack size: {}", props.robot_backpack_size));

            let backpack_size = vec2(390.0, 400.0);
            let backpack_item_size = vec2(380.0, 22.0);
            widgets::Group::new(
                hash!("backpack_contents"),
                backpack_size
            )
            .layout(Layout::Vertical)
            .ui(ui, |ui| {
                for (index, (item, amount)) in props.robot_backpack_contents.iter().enumerate() {
                    widgets::Group::new(
                        hash!("backpack_item", index, amount),
                        backpack_item_size
                    )
                    .layout(Layout::Horizontal)
                    .ui(ui, |ui| {  
                        ui.label(None, &format!("{}", item));
                        ui.same_line(150.0);
                        ui.label(None, &format!("{}", amount));
                    });
                }
            });

            ui.label(None, format!("Discoverable tiles: {}", props.discoverable_tiles).as_str());
            ui.label(None, format!("Score: {}", props.robot_score).as_str());
            ui.label(None, format!("Time of day: {:?}", props.time_of_day).as_str());
            ui.label(None, format!("Time clock: {}", props.time_of_day_string).as_str());
            ui.label(None, format!("Weather: {}", match props.weather_condition {
                WeatherType::Sunny => "Sunny",
                WeatherType::Rainy => "Rainy",
                WeatherType::Foggy => "Foggy",
                WeatherType::TropicalMonsoon => "Tropical moonsoon",
                WeatherType::TrentinoSnow => "Trentino's snow",
                }).as_str()
            );
        });
    }

    fn show_stats(&self) {
        let position = vec2(0.0, 0.0);
        let size = vec2(200.0, 200.0);

        widgets::Window::new(
            hash!("stats_window"), 
            position, 
            size
        )
        .label("Statistics")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "--- MACROQUAD ---");
            ui.label(None, &format!("FPS: {}", get_fps()));
            ui.label(None, &format!("Texture count: {}", textures_count()));

            ui.label(None, "--- SYSTEM ---");
            //let os = linux_os_release().unwrap_or_default();
            ui.label(None, &format!("OS: {}", os_release().unwrap_or("unknown".to_string())));
            ui.label(None, &format!("CPU cores: {}", cpu_num().unwrap_or(0)));
            ui.label(None, &format!("CPU speed: {} MHz", cpu_speed().unwrap_or(0)));
            let ram = mem_info().unwrap_or(MemInfo {
                total: 0,
                free: 0,
                avail: 0,
                buffers: 0,
                cached: 0,
                swap_total: 0,
                swap_free: 0
            });
            ui.label(None, &format!("RAM: {}/{} MB", (ram.total - ram.free) / 10_u64.pow(3), ram.total / 10_u64.pow(3)));
        });
    }

    fn show_help(&self) {
        let position = vec2(0.0, self.viewport_height - 200.0);
        let size = vec2(300.0, 200.0);

        widgets::Window::new(
            hash!("help_window"), 
            position,
            size
        )
        .label("Help")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("WASD + mouse to move"));
            ui.label(None, &format!("Toggle mouse grab: G"));
            ui.label(None, &format!("Toggle tile info window: I"));
            ui.label(None, &format!("Toggle statistics window: F3"));
            //ui.label(None, &format!("WIP - Take screenshot: F2"));
            ui.label(None, &format!("Camera mode: C"));
            ui.label(None, &format!("Exit: Esc"));
        });
    }

    fn show_exit_dialog(&mut self) {
        let position = vec2(self.viewport_width / 2.0 - 100.0, self.viewport_height / 2.0 - 50.0);
        let size = vec2(200.0, 100.0);

        widgets::Window::new(
            hash!("exit_dialog"),
            position,
            size
        )
        .label("Exit")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Do you really want to quit?");
            ui.separator();
            ui.same_line(60.0);
            if ui.button(None, "Yes") {
                self.exit = true;
            }
            ui.same_line(120.);
            if ui.button(None, "No") {
                if self.old_grab_status {
                    self.toggle_mouse_grab();
                }
                self.mouse_grabbed_flag = true;
                self.quit_requested = false;
            }
        });
    }

    fn show_tile_info(&self, props: &UIProps) {
        let position = vec2(self.viewport_width / 2.0 - 150.0, 0.0);
        let size = vec2(300.0, 100.0);

        widgets::Window::new(
            hash!("tile_info_window"), 
            position,
            size
        )
        .label("Tile")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            let (x, z) = props.robot_coordinates;

            if let Some(tile) = &props.explored_world_map[x][z] {
                ui.label(None, format!("Tile type: {:?}", tile.tile_type).as_str());
                ui.label(None, format!("Tile type: {}", tile.content).as_str());
            }
        });
    }

    pub(crate) fn exit(&self) -> bool {
        self.exit
    }

    pub(super) fn is_day_light_cycle_on(&self) -> bool {
        self.daylight_cycle
    }

    pub(super) fn render(&mut self, props: UIProps) {
        draw_text("Press H for help", 0.0, self.viewport_height - 80.0, 30.0, GREEN);

        self.show_game_info(&props);
        
        if self.show_tile_info {
            self.show_tile_info(&props);
        }
        if self.show_stats {
            self.show_stats();
        }
        if self.show_help {
            self.show_help();
        }
        if self.quit_requested {
            if self.mouse_grabbed_flag {
                self.mouse_grabbed_flag = false;
                self.old_grab_status = self.is_mouse_grabbed;
            }
            if self.is_mouse_grabbed {
                self.toggle_mouse_grab();
            }
            self.show_exit_dialog();
        }
    }
}