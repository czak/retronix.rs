use std::collections::VecDeque;
use renderer::Renderer;

mod states;

pub enum Event {
    Tick,
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
}

pub enum Transition {
    Push(Box<State>),
    Pop(usize),
    Replace(Box<State>),
    None,
}

impl Transition {
    /// Returns true if navigation succeeded
    pub fn navigate(self, states: &mut Vec<Box<State>>) -> bool {
        match self {
            Transition::Push(next) => {
                states.push(next);
                true
            },
            Transition::Replace(next) => {
                *states.last_mut().unwrap() = next;
                true
            },
            Transition::Pop(count) => {
                if count >= states.len() {
                    return false;
                }
                for _ in 0..count {
                    states.pop();
                }
                true
            }
            _ => true
        }
    }
}

pub trait State {
    fn update(&mut self) -> Transition;
    fn render(&self, renderer: &mut Renderer);
    fn render_parent(&self) -> bool;
    fn handle_event(&mut self, event: Event) -> Transition;
}

pub struct Game {
    events: VecDeque<Event>,
    states: Vec<Box<State>>,
}

// NOTE: Assumes there will be at least one state
// (see unwrap in Game::current_state)
impl Game {
    pub fn render(&mut self, renderer: &mut Renderer) {
        let first = self.states.iter().rposition(|state| {
            !state.render_parent()
        }).unwrap();

        for state in self.states.iter().skip(first) {
            state.render(renderer);
        }
    }

    pub fn update(&mut self) -> bool {
        self.current_state().update().navigate(&mut self.states)
    }

    pub fn push_event(&mut self, e: Event) {
        self.events.push_back(e);
    }

    pub fn handle_event(&mut self) -> bool {
        if self.events.is_empty() { return true; }

        let event = self.events.pop_front().unwrap();
        self.current_state().handle_event(event).navigate(&mut self.states)
    }

    fn current_state(&mut self) -> &mut Box<State> {
        self.states.last_mut().unwrap()
    }
}

pub fn init() -> Game {
    Game {
        events: VecDeque::new(),
        states: vec![
            Box::new(states::WelcomeState {}),
            // Box::new(states::PlayState::new(1, 0, 3)),
            // Box::new(states::GameOverState {}),
        ],
    }
}
