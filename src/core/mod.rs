



use std::{pin::Pin, any::Any};


pub trait Ticks {
    fn tick(&mut self);
    fn other(&mut self);
}


pub trait Clock {

}



pub trait System: Any {
    fn start(&mut self);
    fn stop(&mut self);
}

#[derive(Default)]
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

            if let Some(ticks2) = ticks_as.downcast_ref::<&dyn Ticks>() {
                self.ticks.push(*ticks2);
            }
        }
    }

    pub fn tick_all_st(&mut self) {
        for tick_src in self.ticks.iter_mut() {
            // This unsafe block is required because we can't guarantee that there is no aliasing
            // between mutable references in the ticks vector.
            // However, this is safe in this specific case because the tick_all function
            // doesn't have access to the Systems struct or the ticks vector during iteration.
            let tick_raw = *tick_src as *const dyn Ticks;
            let tick: &mut dyn Ticks = unsafe { &mut *(tick_raw as *mut dyn Ticks) };
            tick.tick();
        }
    }


    pub fn other_all_st(&mut self) {
        for tick_src in self.ticks.iter_mut() {
            // This unsafe block is required because we can't guarantee that there is no aliasing
            // between mutable references in the ticks vector.
            // However, this is safe in this specific case because the tick_all function
            // doesn't have access to the Systems struct or the ticks vector during iteration.
            let tick_raw = *tick_src as *const dyn Ticks;
            let tick: &mut dyn Ticks = unsafe { &mut *(tick_raw as *mut dyn Ticks) };
            tick.other();
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