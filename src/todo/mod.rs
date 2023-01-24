mod todo;
mod todo_utils;

use std::{
    cell::RefCell,
    io::{self, stdout, Stdout},
    rc::Rc,
};

use cli_printer::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel},
        view::SectionsView,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::{self, Input, TextBlock},
};
use crossterm::{execute, style::Print};

use crate::{cli::TodoCommands, constants, utils};

use self::{
    todo::{Todo, TodoState},
    todo_utils::table,
};

pub struct TodoProgram;

impl TodoProgram {
    pub fn run(todo_command: &TodoCommands) -> io::Result<()> {
        let mut stdout = stdout();
        match todo_command {
            TodoCommands::List => {
                let res = list_todo(&mut stdout);
                if let Err(e) = res {
                    println!("{}", e.to_string())
                }
                Ok(())
            }
            TodoCommands::Create => {
                let res = create_todo(&mut stdout);
                if let Err(e) = res {
                    println!("{}", e.to_string())
                }
                Ok(())
            }
        }
    }
}

fn list_todo(stdout: &mut Stdout) -> io::Result<()> {
    let mut program_path = utils::get_folder_program()?;
    program_path.push(constants::NAME_TODOS_FILE);

    let todo_content = utils::get_content_file(&program_path)?;

    if todo_content.trim().len() == 0 {
        return Ok(());
    }
    let todos_collect: Vec<Vec<String>> = todo_content
        .trim()
        .lines()
        .map(|f| {
            let todo = Todo::try_from(f).unwrap();
            vec![todo.icon, todo.description, todo.status.to_string()]
        })
        .collect();

    let content_table = table(
        todos_collect,
        vec!["Icon".to_string(), "Todo".to_string(), "Status".to_string()],
    );

    for column in &content_table {
        for row in column {
            execute!(stdout, Print(""), Print(row), Print(" "))?;
        }
        execute!(stdout, Print(" \n"))?;
    }

    Ok(())
}

fn create_todo(stdout: &mut Stdout) -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    let mut input: Input<Rc<RefCell<String>>> = widgets::Input::new(
        IconAndLabel(ICON_QUESTION, "Type the todo: "),
        IconAndLabel(ICON_CHECK, "Type the todo: "),
    );

    input.after(|local, global_data| {
        if local.complete_input {
            *(*global_data).borrow_mut() = local.input.clone();
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut text: TextBlock<Rc<RefCell<String>>> = TextBlock::new("Adding todo...");
    text.after(|local, global| {
        let mut program_path = utils::get_folder_program().unwrap();
        program_path.push(constants::NAME_TODOS_FILE);

        let todo_res = utils::get_content_file(&program_path);
        let mut todo_content = match todo_res {
            Ok(s) => s,
            Err(e) => {
                local.text.push_str(&format!("\n{}", e.to_string()));
                return Action::Exit;
            }
        };

        todo_content.push_str(&format!(
            "\n{}:{}",
            (*global).borrow().as_str(),
            TodoState::NoStarted.to_i8()
        ));

        if let Err(e) = utils::write_file(program_path, &todo_content.trim()) {
            local.text.push_str(&format!("\n{}", e.to_string()));
            return Action::Exit;
        }
        local.text.push_str("\nTodo added correctly!");
        Action::Next
    });

    let mut render_view = SectionsView::new(String::new());
    render_view.child(input);
    render_view.child(text);

    render_view.render(stdout)?;

    Ok(())
}
