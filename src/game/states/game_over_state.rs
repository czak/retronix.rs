use game::{State, Event, Transition};
use renderer::{Renderer, Color};

pub struct GameOverState {
}

impl State for GameOverState {
    fn update(&mut self) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let msg = [
            "╔═══════════╗",
            "║ GAME OVER ║",
            "╚═══════════╝",
        ];

        for (y, line) in msg.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                renderer.put_cell(x as u16 + 34, y as u16 + 11, c, Color::White);
            }
        }
    }

    fn render_parent(&self) -> bool {
        true
    }

    fn handle_event(&mut self, _event: Event) -> Transition {
        Transition::Pop(2)
    }
}
