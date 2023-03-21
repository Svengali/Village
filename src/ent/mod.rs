




use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};




use std::sync::atomic::{AtomicU64, Ordering};

use crate::com;

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Id(u64);

static S_CURRENT_ID: AtomicU64 = AtomicU64::new(1024);

static s_next_id: AtomicUsize = AtomicUsize::new( 1024 );

impl Id {
    pub fn next() -> Id {

        let next_id = s_next_id.fetch_add(1, Ordering::Acquire);

        Id {
            id: next_id,
        }
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
        }
    }
}

#[derive(Debug, Default)]
pub struct World {
    map: HashMap<Id, Entity>,
}

impl World
{

    pub fn new() -> World {
        World {
            ..Default::default()
        }
    }

    pub fn add(&mut self, ent: Entity ) {
        self.map.insert(ent.id, ent);
    }

    pub fn lookup(&self, id: Id) -> Option<&Entity> {
        self.map.get( &id )
    }

}