use std::collections::HashMap;



#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Id
{
    id: i32,
}





#[derive(Clone, Copy, Debug, Default)]
pub struct Entity
{

    pub id: Id,

}

#[derive(Clone, Debug, Default)]
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