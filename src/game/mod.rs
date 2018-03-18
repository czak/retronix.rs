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
    fn update(&mut self) -> Option<Box<State>>;
    fn render(&mut self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: Event) -> Option<Box<State>>;
}

pub struct Game {
    events: VecDeque<Event>,
    states: Vec<Box<State>>,
}

impl Game {
    pub fn render(&mut self, renderer: &mut Renderer) {
        if let Some(state) = self.states.last_mut() {
            state.render(renderer);
        }
    }

    pub fn update(&mut self) {
        let transition = {
            let current_state = self.states.last_mut();
            current_state.and_then(|s| s.update())
        };

        if let Some(next_state) = transition {
            self.states.push(next_state);
        }
    }

    pub fn push_event(&mut self, e: Event) {
        self.events.push_back(e);
    }

    pub fn handle_event(&mut self) {
        if let Some(event) = self.events.pop_front() {
            if let Some(next_state) = self.states.last_mut().and_then(|s| s.handle_event(event)) {
                self.states.push(next_state);
            }
        }
    }
}

pub fn init() -> Game {
    Game {
        events: VecDeque::new(),
        states: vec![Box::new(states::PlayState::new())],
    }
}
