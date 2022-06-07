use crossterm::event::KeyCode;
use tokio::sync::mpsc::channel;
use tui::style::Color;

use serde::Deserialize;

pub struct App {
    pub todos: Vec<Todo>,
    pub split: bool,
    pub should_quit: bool,
    pub color: Color,
    rx: tokio::sync::mpsc::Receiver<StateChange>,
    sx: tokio::sync::mpsc::Sender<StateChange>,
}

impl App {
    pub fn new() -> Self {
        let (sx, rx) = channel(100);

        Self {
            todos: vec![],
            split: false,
            should_quit: false,
            color: Color::Green,
            rx,
            sx,
        }
    }

    pub fn tick(&mut self) {
        while let Ok(change) = self.rx.try_recv() {
            match change {
                StateChange::NewTodos(todos) => self.todos = todos,
            };
        }
    }

    pub fn take_action(&mut self, key: KeyCode) {
        match App::map_action(key) {
            Some(Action::Quit) => self.should_quit = true,
            _ => (),
        };
    }

    pub fn init(&mut self) {
        let tx = self.sx.clone();
        tokio::spawn(async move {
            let todos = reqwest::Client::new()
                .get("https://rocket-postgres-api.herokuapp.com/hello")
                .header("Authorization", "Bearer valid")
                .send()
                .await
                .unwrap()
                .json::<Vec<Todo>>()
                .await
                .unwrap();

            tx.send(StateChange::NewTodos(todos)).await.unwrap();
        });
    }

    fn map_action(key: KeyCode) -> Option<Action> {
        match key {
            KeyCode::Char('q') => Some(Action::Quit),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Todo {
    title: String,
    body: String,
    published: bool,
}

#[derive(Debug)]
pub enum StateChange {
    NewTodos(Vec<Todo>),
}

enum Action {
    Quit,
}
