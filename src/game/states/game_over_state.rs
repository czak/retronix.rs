use game::{State, Event, Transition};
use renderer::Renderer;

pub struct GameOverState {
}

impl State for GameOverState {
    fn update(&mut self) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let msg = "GAME OVER";
        for (i, c) in msg.chars().enumerate() {
            renderer.put_cell(i as u16, 0, c);
        }
    }

    fn render_parent(&self) -> bool {
        true
    }

    fn handle_event(&mut self, _event: Event) -> Transition {
        Transition::None
    }
}
