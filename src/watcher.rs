use crate::constants::RESSOURCE_PATH;
use crate::helpers;
use crate::world::World;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileWatcher {
    // sender: Sender<DebouncedEvent>,
    receiver: Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new() -> Self {
        // Watch the ressources folder every 2 secs.
        let (sender, receiver) = channel();
        let mut watcher: RecommendedWatcher =
            Watcher::new(sender, Duration::from_secs(2)).unwrap();

        watcher
            .watch(RESSOURCE_PATH, RecursiveMode::NonRecursive)
            .unwrap();

        Self { receiver }
    }

    pub fn update(&self, world: &mut World) {
        if self.receiver.try_recv().is_ok() {
            world.entities = helpers::load_entities();
        } else {
            dbg!(&self.receiver.try_recv());
        }
    }
}
