



use std::{pin::Pin, any::Any};


pub trait Ticks {
    fn tick(&mut self);
}


pub trait Clock {

}



pub trait System: Any {
    fn start(&mut self);
    fn stop(&mut self);
}


pub struct Systems<'a> {
    systems: Vec<&'a dyn System>,
    ticks: Vec<&'a dyn Ticks>,
}

impl<'a> Systems<'a> {
    pub fn new() -> Self {
        Systems {
            systems: Vec::new(),
            ticks: Vec::new(),
        }
    }

    pub fn register<T>(&mut self, system: &'a T)
    where
        T: System + 'a,
    {
        self.systems.push(system);

        // Check if the system implements the Ticks trait
        if let Some(ticks) = system.as_any().downcast_ref::<T>() {

            let ticks_as = ticks.as_any();

            if let Some(ticks) = ticks_as.downcast_ref::<&dyn Ticks>() {
                self.ticks.push(*ticks);
            }
        }
    }
}

// Add as_any method for the System trait
pub trait SystemExt: System {
    fn as_any(&self) -> &dyn Any;
}

impl<T: System + 'static> SystemExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}