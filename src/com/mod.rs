

use std::{any::*, collections::{HashMap, hash_map::DefaultHasher}, hash::Hash};

use std::any::Any;

trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Default, Copy, Clone)]
pub struct Renderable {
    pub test: u64,

}

impl Component for Renderable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


#[derive(Default)]
pub struct Components {
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Components {
    fn new() -> Components {
        Components {
            components: HashMap::new(),
        }
    }

    pub fn add<T: 'static + Component>(&mut self, component: T) {

        let type_id = std::any::TypeId::of::<T>();

        self.components.insert(type_id, Box::new(component));
    }


    pub fn get<T: 'static + Component>(&self) -> Option<&T> {
        let type_id = std::any::TypeId::of::<T>();

        let val = self.components.get(&type_id);

        val?.as_any().downcast_ref::<T>()

        /*
        match val {
            None => None,
            Some(c) => {
                let t: &T = match c.as_any().downcast_ref::<T>() {
                    Some(n) => n,
                    None => panic!("&a isn't a B!"),
                };

                Some( t )
            }
        }
        */
    }

    pub fn get_mut<T: 'static + Component>(&mut self) -> Option<&mut T> {
        let type_id = std::any::TypeId::of::<T>();

        let val = self.components.get_mut(&type_id);

        val?.as_any_mut().downcast_mut::<T>()
    }

}