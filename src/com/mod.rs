

use std::{any::*, collections::{HashMap, hash_map::DefaultHasher}, hash::Hash};

use std::any::Any;

trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct RenderCom;

impl Component for RenderCom {
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

    fn add<T: 'static + Component>(&mut self, component: T) {

        let type_id = std::any::TypeId::of::<T>();

        self.components.insert(type_id, Box::new(component));
    }


    fn get<T: 'static + Component>(&self, entity_id: usize) -> Option<&T> {
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

    fn get_mut<T: 'static + Component>(&mut self, entity_id: usize) -> Option<&mut T> {
        let type_id = std::any::TypeId::of::<T>();

        let val = self.components.get_mut(&type_id);

        val?.as_any_mut().downcast_mut::<T>()
    }

}