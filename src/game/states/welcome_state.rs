use game::{State, Event, Renderer};

pub struct WelcomeState {
}

impl State for WelcomeState {
    fn update(&mut self) -> Option<Box<State>> {
        None
    }

    fn render(&mut self, renderer: &mut Renderer) {
        let msg = "Press → to play.";
        for (i, c) in msg.chars().enumerate() {
            renderer.put_cell(i as u16, 0, c);
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Box<State>> {
        match event {
            Event::Right => {
                let state = super::PlayState::new();
                Some(Box::new(state))
            },
            _ => None,
        }
    }
}
