use game::{State, Event, Renderer, Transition};

#[allow(dead_code)]
pub struct WelcomeState {
}

impl State for WelcomeState {
    fn update(&mut self) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let msg = "Press → to play.";
        for (i, c) in msg.chars().enumerate() {
            renderer.put_cell(i as u16, 0, c);
        }
    }

    fn handle_event(&mut self, event: Event) -> Transition {
        match event {
            Event::Right => {
                let state = super::PlayState::new(1, 0, 3);
                Transition::Push(Box::new(state))
            },
            _ => Transition::None,
        }
    }
}
