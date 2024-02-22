use std::{cell::RefCell, rc::Rc};

use channel::Channel;
use macroquad::prelude::*;
use robotics_lib::{runner::Runnable, world::world_generator::Generator};
use gui::GUI;
use runner_wrapper::RunnerWrapper;

mod gui;
mod runner_wrapper;
pub mod channel;

pub struct Visualizer {
    runner: RunnerWrapper,
    gui: GUI,
    channel: Rc<RefCell<Channel>>,
}

impl Visualizer {
    pub fn new(robot: Box<dyn Runnable> , world_generator: impl Generator, world_size: usize, channel: Rc<RefCell<Channel>>) -> Self {        
        let tick_time = Rc::new(RefCell::new(0.5));
        
        Self {
            runner: RunnerWrapper::new(robot, world_generator, Rc::clone(&tick_time)),
            gui: GUI::new(world_size, Rc::clone(&tick_time)),
            channel,
        }
    }

    pub async fn start(&mut self) {
        //set_pc_assets_folder("assets");
        self.gui.ui.toggle_mouse_grab();

        loop {
            self.gui.handle_input();
            if self.gui.ui.exit() {
                break;
            }

            self.runner.tick();
            
            self.gui.render(self.channel.borrow().receive());
    
            next_frame().await
        }
    }
}