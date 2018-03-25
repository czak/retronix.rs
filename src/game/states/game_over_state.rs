use game::{State, Event, Renderer, Transition};

pub struct GameOverState {
}

impl State for GameOverState {
    fn update(&mut self) -> Transition {
        Transition::None
    }

    fn render(&mut self, renderer: &mut Renderer) {
        let msg = "GAME OVER";
        for (i, c) in msg.chars().enumerate() {
            renderer.put_cell(i as u16, 0, c);
        }
    }

    fn handle_event(&mut self, _event: Event) -> Transition {
        Transition::None
    }
}
