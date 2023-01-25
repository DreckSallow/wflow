pub mod todo;
pub mod todo_utils;

use std::{
    cell::RefCell,
    io::{self, stdout, Stdout},
    rc::Rc,
};

use cli_printer::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel, RenderWidget},
        view::SectionsView,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::{self, Input, TextBlock},
};
use crossterm::{
    execute,
    style::{Print, Stylize},
};

use crate::{cli::TodoCommands, constants, custom_widgets, utils};

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
            TodoCommands::Check => {
                let res = change_todo(&mut stdout);
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
    let mut input: Input<Rc<RefCell<(String, Option<String>)>>> = widgets::Input::new(
        IconAndLabel(ICON_QUESTION, "Type the todo: "),
        IconAndLabel(ICON_CHECK, "Type the todo: "),
    );

    input.after(|local, global_data| {
        if local.complete_input {
            let mut input_mutable = local.input.clone();
            input_mutable.push_str(":0");
            let exist_error = Todo::try_from(input_mutable.as_str()).is_err();
            if exist_error {
                (*global_data).borrow_mut().1 =
                    Some("Could not create 'todo', maybe you wrote ':' in the input".to_string());
            }

            (*global_data).borrow_mut().0 = local.input.clone();
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut text: TextBlock<Rc<RefCell<(String, Option<String>)>>> =
        TextBlock::new("Adding todo...");
    text.before(|local, global_state| {
        if let Some(ref msg) = (*global_state).borrow().1 {
            local.text = msg.to_string();
            return RenderWidget::No;
        }

        RenderWidget::Yes
    });

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
            (*global).borrow().0.as_str(),
            TodoState::NoStarted.to_i8()
        ));

        if let Err(e) = utils::write_file(program_path, &todo_content.trim()) {
            local.text.push_str(&format!("\n{}", e.to_string()));
            return Action::Exit;
        }
        local.text.push_str("\nTodo added correctly!");
        Action::Next
    });

    let mut render_view = SectionsView::new((String::new(), None));
    render_view.child(input);
    render_view.child(text);

    render_view.render(stdout)?;

    Ok(())
}

fn change_todo(stdout: &mut Stdout) -> io::Result<()> {
    let mut program_path = utils::get_folder_program()?;
    program_path.push(constants::NAME_TODOS_FILE);

    let todo_content = utils::get_content_file(&program_path)?;

    if todo_content.trim().len() == 0 {
        return Ok(());
    }
    let todos_collect: Vec<Todo> = todo_content
        .trim()
        .lines()
        .map(|f| Todo::try_from(f).unwrap())
        .collect();

    let todo_completed = Print(format!("{}: Completed", "[x]".cyan().bold()));
    let todo_blank = Print(format!("{}: Not started", "[ ]".cyan().bold()));
    let change_behavior = Print(format!("{}: Change the icon", "←/→".cyan().bold()));

    let legend_text = TextBlock::new(&format!(
        "{} - {}, {}",
        todo_completed, todo_blank, change_behavior
    ));

    let mut render_todos: custom_widgets::CheckList<Rc<RefCell<Vec<Todo>>>> =
        custom_widgets::CheckList::new(todos_collect);

    render_todos.after(|local_state, global_state| {
        if local_state.is_selected {
            *(*global_state).borrow_mut() = local_state.todos.clone();
            let todos = &*(*global_state)
                .borrow()
                .iter()
                .filter(|t| t.status == TodoState::Completed)
                .map(|t| t.clone())
                .collect::<Vec<Todo>>();

            if todos.len() == 0 {
                return Action::Exit;
            }

            return Action::Next;
        }
        Action::KeepSection
    });

    let mut confirmation: widgets::ListSelected<Rc<RefCell<Vec<Todo>>>> =
        widgets::ListSelected::new(vec!["Yes", "No"]);

    confirmation.add_text_init(ICON_QUESTION, "Delete the completed todos?: ");
    confirmation.after(move |state, global_state| {
        if state.is_selected {
            let todos = &*(*global_state).borrow();

            let mut content = String::new();

            let delete_todo = state.offset != state.length - 1; // SI selection!

            for todo in todos {
                if delete_todo && todo.status == TodoState::Completed {
                    continue;
                }
                content.push_str(&format!("{}:{}\n", todo.description, todo.status.to_i8()));
            }

            let res = utils::write_file(&program_path, content.trim());
            if let Err(e) = res {
                println!("{}", format!("{}", e.to_string()));
                return Action::Exit;
            }

            return Action::Next;
        }

        Action::KeepSection
    });

    let mut section_view = SectionsView::new(vec![]);
    section_view.child(legend_text);
    section_view.child(render_todos);
    section_view.child(confirmation);
    section_view.render(stdout)?;

    Ok(())
}
