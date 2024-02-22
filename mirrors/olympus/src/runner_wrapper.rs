use std::{cell::RefCell, rc::Rc};

use macroquad::time::get_time;
use robotics_lib::{runner::{Runnable, Runner}, world::world_generator::Generator};

pub(super) struct RunnerWrapper {
    runner: Runner,
    last_time: f64,
    current_time: f64,
    tick_time: Rc<RefCell<f32>>
}

impl RunnerWrapper {
    pub(super) fn new(robot: Box<dyn Runnable>, mut world_generator: impl Generator, tick_time: Rc<RefCell<f32>>) -> Self {
        Self {
            runner: Runner::new(robot, &mut world_generator).expect("Error creating runner"),
            last_time: get_time(),
            current_time: get_time(),
            tick_time
        }
    }

    pub(super) fn tick(&mut self) {
        self.current_time = get_time();
        
        if (self.current_time - self.last_time) > *self.tick_time.borrow() as f64 {
            self.runner.game_tick().expect("Error during game tick");
            self.last_time = self.current_time;
        }
    }
}