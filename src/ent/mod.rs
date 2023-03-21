

use std::collections::HashMap;

use std::sync::atomic::{AtomicU64, Ordering};

use crate::com;

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Id(u64);

static S_CURRENT_ID: AtomicU64 = AtomicU64::new(1024);




#[derive(Default)]
pub struct Entity
{

    pub id: Id,

    pub com: com::Components,


}

impl Entity {
    pub fn new() -> Entity {
        
        let id = Id(S_CURRENT_ID.fetch_add(1, Ordering::AcqRel));

        Entity {
            id,
            ..Default::default()
        }
    }
}


#[derive(Default)]
pub struct World
{

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