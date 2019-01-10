use glutin::{Event, EventsLoop, WindowEvent};

pub struct GameController {
    event_loop: EventsLoop,
}

#[derive(PartialEq)]
#[allow(unused)]
pub enum Input {
    Direction,
    CloseRequested,
}

impl GameController {
    pub fn new(event_loop: EventsLoop) -> Self {
        Self { event_loop }
    }

    pub fn pull(&mut self) -> Option<Input> {
        let mut input = None;
        self.event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => input = Some(Input::CloseRequested),
            _ => (),
        });

        input
    }
}
