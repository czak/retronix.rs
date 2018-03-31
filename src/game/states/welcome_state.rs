use game::{State, Event, Transition};
use renderer::{Renderer, Color};

#[allow(dead_code)]
pub struct WelcomeState {
}

impl State for WelcomeState {
    fn update(&mut self) -> Transition {
        Transition::None
    }

    fn render(&self, renderer: &mut Renderer) {
        let msg = [
            "░░░░░░░░  ░░░░░░░░░ ░░░░░░░░ ░░░░░░░░   ░░░░░░░  ░░░░   ░░░ ░░░░░░░░ ░░░   ░░░",
            "░▒░   ░▒░ ░▒░          ░░    ░▒░   ░▒░ ░▒░   ░▒░ ░▒░▒░  ░▒░    ░░    ░▒░   ░▒░",
            "▒░▒   ▒░▒ ▒░▒          ▒▒    ▒░▒   ▒░▒ ▒░▒   ▒░▒ ░▒░▒░░ ▒░▒    ▒▒     ▒░▒ ▒░▒ ",
            "▒█▒▒▒▒█░  ▒█▒▒▒▒█      ▒▒    ▒█▒▒▒▒█░  ▒█▒   ▒░▒ ▒█▒ ▒░ ▒█▒    ▒▒      ▒█▒░▒  ",
            "▒█▒   ▒█▒ ▒█▒          ▒▒    ▒█▒   ▒█▒ ▒█▒   ▒█▒ ▒█▒  ▒██▒█    ▒▒     ▒█▒ ▒█▒ ",
            "█▒█   █▒█ █▒█          ██    █▒█   █▒█ █▒█   █▒█ █▒█   ██▒█    ██    █▒█   █▒█",
            "███   ███ █████████    ██    ███   ███  ███████  ███    ███ ████████ ███   ███",
        ];

        for (y, line) in msg.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let color = match c {
                    '░' => Color::Magenta,
                    '▒' => Color::Cyan,
                    '█' => Color::White,
                    _ => Color::White,
                };
                renderer.put_cell(x as u16, y as u16, c, color);
            }
        }

        for (i, c) in "Made by Lukasz Adamczak".chars().enumerate() {
            renderer.put_cell((i + 30) as u16, 10, c, Color::White);
        }

        for (i, c) in "Based on Xonix by Ilan Rav & Dani Katz".chars().enumerate() {
            renderer.put_cell((i + 22) as u16, 11, c, Color::White);
        }
    }

    fn render_parent(&self) -> bool {
        false
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
