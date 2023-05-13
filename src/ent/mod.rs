




use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::com;

use crate::core;


#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Id(u64);

static S_CURRENT_ID: AtomicU64 = AtomicU64::new(1024);

//static s_next_id: AtomicUsize = AtomicUsize::new( 1024 );

impl Id {
    pub fn next() -> Id {

        let next_id = S_CURRENT_ID.fetch_add(1, Ordering::Acquire);

        Id(next_id)
    }
}


#[derive(Default)]
pub struct Entity
{

    pub id: Id,

    pub com: com::Components,


}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            id: Id::next(),
            ..Default::default()
        }
    }
}



pub struct World {
    pub systems: core::Systems,
    map: HashMap<Id, Entity>,
}

impl World
{

    pub fn new() -> World {
        World {
            systems: core::Systems::new(),
            ..Default::default()
        }
    }

    pub fn _test() {
    }

    pub fn add(&mut self, ent: Entity ) {
        self.map.insert(ent.id, ent);
    }

    pub fn _lookup(&self, id: Id) -> Option<&Entity> {
        self.map.get( &id )
    }

}

impl Default for World {
    fn default() -> Self {
        World {
            systems: core::Systems::new(),
            map: HashMap::new(),
        }
    }
}
