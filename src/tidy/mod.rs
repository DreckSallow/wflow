mod proyects_db;

use std::{
    cell::RefCell,
    env, fs,
    io::{self, stdout, Stdout},
    path::{Path, PathBuf},
    rc::Rc,
};

use cli_printer::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel},
        view,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::{self, Input, ListSelected, TextBlock},
};
use crossterm::{
    execute,
    style::{Print, Stylize},
};

use crate::cli::TidyCommands;

use self::proyects_db::get_proyects_content;

fn canonicalize_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = path.as_ref().canonicalize()?.display().to_string();
    let slice_path = if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    };
    let mut new_path = PathBuf::new();
    new_path.push(slice_path);
    Ok(new_path)
}

pub struct TidyProgram;

impl TidyProgram {
    pub fn run(tidy_command: &TidyCommands) -> io::Result<()> {
        let mut stdout = stdout();

        match tidy_command {
            TidyCommands::Open => todo!(),
            TidyCommands::Add { path } => {
                let _r = add_proyect(&mut stdout, path);

                Ok(())
            }
            TidyCommands::New => {
                let _r = new_proyect(&mut stdout);
                Ok(())
            }
            TidyCommands::Remove => {
                let _r = remove_proyect(&mut stdout);
                Ok(())
            }
        }
    }
}

fn add_proyect(stdout: &mut Stdout, path: &PathBuf) -> io::Result<()> {
    let path_to_save = canonicalize_path(path)?;
    proyects_db::append_to_first_proyect(&path_to_save)?;
    execute!(
        stdout,
        Print(ICON_CHECK.green()),
        Print("Added "),
        Print(path_to_save.display().to_string().green()),
        Print(" successfully")
    )?;
    Ok(())
}

fn new_proyect(stdout: &mut Stdout) -> io::Result<()> {
    let mut input_widget: Input<Rc<RefCell<String>>> = Input::new(
        IconAndLabel(ICON_QUESTION, "Name of the new proyect: "),
        IconAndLabel(ICON_CHECK, "Name of the new proyect: "),
    );

    input_widget.after(move |input_state, global_state| {
        if input_state.complete_input && input_state.input.len() > 0 {
            *(*global_state).borrow_mut() = input_state.input.to_owned();
            return Action::Next;
        }
        Action::KeepSection
    });

    //Render:
    let mut render_view = view::SectionsView::new(String::new());
    render_view.child(input_widget);
    render_view.render(stdout)?;

    let input_content = (&*render_view.global_state).borrow();

    let mut new_path = env::current_dir()?;
    new_path.push(input_content.to_string());
    proyects_db::append_to_first_proyect(&new_path)?;
    fs::create_dir(new_path)?;
    execute!(stdout, Print("\nNew Folder create and save"))?;
    Ok(())
}

#[derive(Default, Clone)]
struct GlobalState {
    proyect_selected: Option<String>,
    remove_folder_proyect: bool,
}

fn remove_proyect(stdout: &mut Stdout) -> io::Result<()> {
    let binding = get_proyects_content()?;
    let mut proyects: Vec<&str> = binding.trim().lines().collect();
    proyects.push("None");

    // List of all proyects to select
    let mut list: ListSelected<Rc<RefCell<GlobalState>>> = ListSelected::new(proyects);
    list.add_text_init(ICON_QUESTION, "Select the project to delete: ");
    list.add_text_final(ICON_CHECK, "Selected option: ");

    list.after(|list_state, global_data| {
        if list_state.is_selected {
            if list_state.offset == list_state.length - 1 {
                return Action::Exit;
            }
            (*global_data).borrow_mut().proyect_selected = list_state.current_option.clone();
            return Action::Next;
        }
        Action::KeepSection
    });

    // List to render: 'Yes' | 'No'
    let mut remove_folder_list: ListSelected<Rc<RefCell<GlobalState>>> =
        widgets::ListSelected::new(vec!["Yes", "No"]);
    remove_folder_list.add_text_init(ICON_QUESTION, "Also delete the folder: ");
    remove_folder_list.add_text_final(ICON_CHECK, "Also delete the folder: ");

    remove_folder_list.after(|list_state, global_state| {
        (*global_state).borrow_mut().remove_folder_proyect =
            if list_state.offset == 0 { true } else { false };

        if list_state.is_selected {
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut remove_text: TextBlock<Rc<RefCell<GlobalState>>> = TextBlock::new("Removing...");

    remove_text.after(|local_state, global_state| {
        let context_state = &(*global_state).borrow_mut();

        let selected_project = match &context_state.proyect_selected {
            Some(p) => p.to_owned(),
            None => {
                local_state.text.push_str("\nNot exist the proyect!");
                return Action::Exit;
            }
        };

        let res = proyects_db::delete_proyect(Path::new(&selected_project));

        if let Err(e) = res {
            local_state.text.push_str(&format!("\n{}", e.to_string()));
            return Action::Exit;
        }
        local_state.text.push_str("\nProject removed!");

        if context_state.remove_folder_proyect {
            let path_opt = &context_state.proyect_selected;

            let path_folder = match path_opt {
                Some(p) => p.to_owned(),
                None => {
                    local_state.text.push_str("\nUnexpected error ocurred");
                    return Action::Exit;
                }
            };

            let path = Path::new(path_folder.as_str());

            if !path.is_dir() {
                local_state.text.push_str("\nThe path is not a folder");
                return Action::Exit;
            }

            if let Err(e) = fs::remove_dir_all(path) {
                local_state.text.push_str(&format!("\n{}", e.to_string()));
                return Action::Exit;
            }
            local_state.text.push_str("\nFolder removed!!");
        }

        Action::Next
    });

    let mut render_view = view::SectionsView::new(GlobalState::default());
    render_view.child(list);
    render_view.child(remove_folder_list);
    render_view.child(remove_text);
    render_view.render(stdout)?;

    Ok(())
}
