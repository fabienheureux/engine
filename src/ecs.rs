use crate::GameState;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

static mut ENTITY_INDEX: usize = 0;
pub type EntityType = usize;

#[derive(Default, Debug)]
struct AnyMap {
    data: HashMap<TypeId, Box<Any>>,
}

impl AnyMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
            .expect("Type not found in the list.")
    }

    pub fn get_opt<T: 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> &mut T {
        self.data
            .get_mut(&TypeId::of::<T>())
            .and_then(|any| any.downcast_mut::<T>())
            .expect("Type not found.")
    }

    pub fn insert<T>(&mut self, item: T)
    where
        T: std::fmt::Debug + 'static,
    {
        self.data
            .insert(TypeId::of::<T>(), Box::new(item) as Box<Any>);
    }

    #[allow(unused)]
    pub fn remove<T: 'static>(&mut self) {
        self.data
            .remove(&TypeId::of::<T>())
            .expect("Failed to removing item, key is not found.");
    }
}

#[derive(Debug, Default)]
pub struct Entity {
    pub id: EntityType,
    components: AnyMap,
}

impl Entity {
    #[allow(unused)]
    pub fn new() -> Self {
        let id = unsafe {
            ENTITY_INDEX += 1;
            ENTITY_INDEX
        };

        Self {
            id,
            components: AnyMap::new(),
        }
    }

    pub fn from_file(id: EntityType) -> Self {
        unsafe {
            if id > ENTITY_INDEX {
                ENTITY_INDEX += id;
            }
        };

        Self {
            id,
            components: AnyMap::new(),
        }
    }

    pub fn get_all_types(&self) -> Vec<&TypeId> {
        let mut list = vec![];

        for key in self.components.data.keys() {
            list.push(key);
        }

        list
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.components.get::<T>()
    }

    pub fn get_opt<T: 'static>(&self) -> Option<&T> {
        self.components.get_opt::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self) -> &mut T {
        self.components.get_mut::<T>()
    }

    #[allow(unused)]
    pub fn remove<T: 'static>(mut self) {
        self.components.remove::<T>();
    }

    pub fn with<T: Debug + 'static>(mut self, component: T) -> Self {
        self.components.insert(component);
        self
    }
}

pub trait System: Debug {

    fn get_targets(&self) -> Vec<TypeId> {
        vec![]
    }
    fn process(&self, _targets: &mut Entity, _state: &mut GameState) {}
}

#[derive(Default, Debug)]
pub struct World {
    entities: Vec<Entity>,
    systems: Vec<Box<System>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            systems: vec![],
        }
    }

    #[allow(unused)]
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn load_entities(&mut self, entities: Vec<Entity>) {
        let mut to_add = vec![];

        entities.into_iter().for_each(|entity| {
            let position = self.entities.iter().position(|e| e.id == entity.id);

            if position.is_some() {
                self.entities[position.unwrap()] = entity;
            } else {
                to_add.push(entity);
            }
        });

        self.entities.extend(to_add);
    }

    pub fn add_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    #[allow(unused)]
    pub fn entities_len(&self) -> usize {
        self.entities.len()
    }

    #[allow(unused)]
    pub fn systems_len(&self) -> usize {
        self.systems.len()
    }

    pub fn get_type<T: 'static>() -> TypeId {
        TypeId::of::<T>()
    }

    pub fn run(&mut self, state: &mut GameState) {
        let Self {
            systems, entities, ..
        } = self;

        entities.iter_mut().for_each(|mut entity| {
            let mut should_process = false;

            systems.iter().for_each(|system| {
                let targets = system.get_targets();
                let types = entity.get_all_types();

                should_process = targets
                    .iter()
                    .all(|target| types.iter().any(|t| *t == target));

                if should_process {
                    system.process(&mut entity, state);
                }
            });
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct CompA {
        foo: bool,
    }
    #[derive(Debug)]
    struct CompB {
        bar: bool,
    }

    #[test]
    fn should_add_entity() {
        let mut world = World::new();

        let comp_a = CompA { foo: true };
        world.add_entity(Entity::new().with::<CompA>(comp_a));
        assert_eq!(world.entities_len(), 1);

        let comp_a = CompA { foo: true };
        let comp_b = CompB { bar: false };
        world.add_entity(
            Entity::new().with::<CompA>(comp_a).with::<CompB>(comp_b),
        );
        assert_eq!(world.entities_len(), 2);
    }

    #[test]
    fn should_add_system() {
        let mut world = World::new();

        #[derive(Debug)]
        struct SystemA;
        impl System for SystemA {}

        #[derive(Debug)]
        struct SystemB;
        impl System for SystemB {}

        world.add_system(SystemA);
        assert_eq!(world.systems_len(), 1);
        world.add_system(SystemB);
        assert_eq!(world.systems_len(), 2);
    }

    #[test]
    fn should_run_system() {
        let mut world = World::new();

        let comp_a = CompA { foo: true };
        world.add_entity(Entity::new().with::<CompA>(comp_a));

        #[derive(Debug)]
        struct SystemA;

        impl System for SystemA {
            fn get_targets(&self) -> Vec<TypeId> {
                vec![World::get_type::<CompA>()]
            }

            fn update(&self, entity: &mut Entity) {
                let c = entity.get_mut::<CompA>();
                assert_eq!(c.foo, true);
                c.foo = false;
                assert_ne!(c.foo, true);
            }
        }

        world.add_system(SystemA);
        world.run();
    }
}
