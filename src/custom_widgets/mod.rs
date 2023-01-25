use cli_printer::{
    core::{
        interfaces::{Widget, WidgetChild},
        utils::{Action, IconAndLabel, RenderWidget},
    },
    styles::ICON_QUESTION,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
};

use crate::todo::todo::{Todo, TodoState};

type AfterCb<T> = dyn FnMut(&mut CheckListData, T) -> Action;

type BeforeCb<T> = dyn FnMut(&mut CheckListData, T) -> RenderWidget;

pub struct CheckList<'a, T> {
    pub label: IconAndLabel<'a>,
    pub local_state: CheckListData,
    cb_before: Box<BeforeCb<T>>,
    cb_after: Box<AfterCb<T>>,
}

pub struct CheckListData {
    pub is_selected: bool,
    pub offset: usize,
    pub current_option: Option<String>,
    pub length: usize,
    pub todos: Vec<Todo>,
}

impl<'a, T: Clone> Widget for CheckList<'a, T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            Print(self.label.0.cyan()),
            Print(self.label.1),
            Print("\n"),
        )?;
        if !self.local_state.is_selected {
            let mut i = 0;
            for todo in &self.local_state.todos {
                let text = format!("{} {}", todo.icon.clone(), todo.description.clone());
                if self.local_state.offset == i {
                    let text_colored = color_todo(&todo.status, &format!("> {}", text));
                    execute!(stdout, Print(text_colored),)?;
                } else {
                    let text_colored = color_todo(&todo.status, &format!("  {}", text));
                    execute!(stdout, Print(text_colored))?;
                }
                execute!(stdout, Print("\n"))?;
                i += 1;
            }

            match event::read()? {
                Event::Key(k) => match k.code {
                    KeyCode::Down => self.next(),
                    KeyCode::Up => self.prev(),
                    KeyCode::Left => {
                        let current_todo = self.get_todo_mut();
                        if let Some(todo) = current_todo {
                            let new_icon = match todo.status {
                                TodoState::Completed => TodoState::NoStarted,
                                TodoState::NoStarted => TodoState::Completed,
                            };
                            todo.change_icon(new_icon)
                        }
                    }
                    KeyCode::Right => {
                        let current_todo = self.get_todo_mut();
                        if let Some(todo) = current_todo {
                            let new_icon = match todo.status {
                                TodoState::Completed => TodoState::NoStarted,
                                TodoState::NoStarted => TodoState::Completed,
                            };
                            todo.change_icon(new_icon)
                        }
                    }
                    KeyCode::Enter => self.local_state.is_selected = true,
                    _ => {}
                },
                _ => {}
            }
            return Ok(());
        }

        let mut i = 0;
        for todo in &self.local_state.todos {
            let text = format!("{} {}", todo.icon.clone(), todo.description.clone());
            if self.local_state.offset == i {
                let text_colored = color_todo(&todo.status, &format!("  {}", text));
                execute!(stdout, Print(text_colored))?;
            } else {
                let text_colored = color_todo(&todo.status, &format!("  {}", text));
                execute!(stdout, Print(text_colored))?;
            }
            execute!(stdout, Print("\n"))?;
            i += 1;
        }

        Ok(())
    }
}

impl<'a, T: Clone> WidgetChild<T> for CheckList<'a, T> {
    fn before_render(&mut self, global_state: T) -> cli_printer::core::utils::RenderWidget {
        (self.cb_before)(&mut self.local_state, global_state)
    }

    fn after_render(&mut self, global_state: T) -> cli_printer::core::utils::Action {
        (self.cb_after)(&mut self.local_state, global_state)
    }
}

impl<'a, T: Clone> CheckList<'a, T> {
    pub fn new(todos: Vec<Todo>) -> Self {
        let length = todos.len();
        Self {
            label: IconAndLabel(ICON_QUESTION, "Change the todos: "),
            local_state: CheckListData {
                todos,
                is_selected: false,
                offset: 0,
                current_option: None,
                length,
            },
            cb_after: Box::new(|_, _| Action::Next),
            cb_before: Box::new(|_, _| RenderWidget::Yes),
        }
    }
    pub fn after(&mut self, cb: impl FnMut(&mut CheckListData, T) -> Action + 'static) {
        self.cb_after = Box::new(cb);
    }
    pub fn before(&mut self, cb: impl FnMut(&mut CheckListData, T) -> RenderWidget + 'static) {
        self.cb_before = Box::new(cb);
    }
    pub fn prev(&mut self) {
        let new_offset = if self.local_state.offset == 0 {
            self.local_state.todos.len() - 1
        } else {
            self.local_state.offset - 1
        };

        self.local_state.current_option = self
            .local_state
            .todos
            .get(new_offset)
            .map(|s| s.description.to_owned());
        self.local_state.offset = new_offset;
    }
    pub fn next(&mut self) {
        let new_offset = if self.local_state.offset >= self.local_state.todos.len() - 1 {
            0
        } else {
            self.local_state.offset + 1
        };

        self.local_state.current_option = self
            .local_state
            .todos
            .get(new_offset)
            .map(|s| s.description.to_owned());
        self.local_state.offset = new_offset;
    }

    pub fn get_todo_mut(&mut self) -> Option<&mut Todo> {
        self.local_state.todos.get_mut(self.local_state.offset)
    }
}

fn color_todo(todo_status: &TodoState, text: &str) -> String {
    match todo_status {
        TodoState::Completed => {
            format!("{}", text.green())
        }
        TodoState::NoStarted => {
            format!("{}", text)
        }
    }
}
