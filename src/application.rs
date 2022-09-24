use std::time::{Duration, Instant};

// use super::settings::Settings;

pub type AppCreator = Box<dyn FnOnce(&ApplicationContext) -> Box<dyn Application>>;

pub trait Application {}

pub struct ApplicationContext {}
