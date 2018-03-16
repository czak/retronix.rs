use std::collections::VecDeque;

mod states;

pub enum Event {
    Tick,
    Quit,
    Up,
    Down,
    Left,
    Right,
}

pub trait Renderer {
    fn put_cell(&mut self, x: u16, y: u16, c: char);
}

pub trait State {
    fn update(&mut self);
    fn render(&mut self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: Event);
}

pub struct Game {
    events: VecDeque<Event>,
    states: Vec<Box<State>>,
}

impl Game {
    pub fn render(&mut self, renderer: &mut Renderer) {
        // TODO: ensure there is a state on top
        self.states[0].render(renderer);
    }

    pub fn update(&mut self) {
        self.states[0].update();
    }

    pub fn push_event(&mut self, e: Event) {
        self.events.push_back(e);
    }

    pub fn handle_event(&mut self) {
        if let Some(event) = self.events.pop_front() {
            self.states[0].handle_event(event);
        }
    }
}

pub fn init() -> Game {
    Game {
        events: VecDeque::new(),
        states: vec![Box::new(states::WelcomeState {})],
    }
}
