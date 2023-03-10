#[derive(Clone)]
pub struct Todo {
    pub description: String,
    pub status: TodoState,
    pub icon: String,
}

impl TryFrom<&str> for Todo {
    type Error = TodoError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.trim().split(":").collect();

        if parts.len() < 2 {
            return Err(TodoError::MissingData);
        }
        let description = parts[0].to_string();

        let status = match parts[1].parse::<i8>() {
            Ok(n) => TodoState::from(n),
            Err(_) => return Err(TodoError::ParseError),
        };

        let icon = match status {
            TodoState::Completed => format!("[x]"),
            TodoState::NoStarted => format!("[ ]"),
        };

        return Ok(Self {
            description,
            status,
            icon,
        });
    }
}

impl Todo {
    pub fn new(desc: &str) -> Self {
        Self {
            description: desc.to_string(),
            status: TodoState::NoStarted,
            icon: String::from("[ ]"),
        }
    }
    pub fn change_icon(&mut self, new_state: TodoState) {
        self.icon = match new_state {
            TodoState::Completed => format!("[x]"),
            TodoState::NoStarted => format!("[ ]"),
        };
        self.status = new_state
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TodoState {
    Completed,
    NoStarted,
}

impl From<i8> for TodoState {
    fn from(n: i8) -> Self {
        match n {
            0 => TodoState::NoStarted,
            1 => TodoState::Completed,
            _ => TodoState::NoStarted,
        }
    }
}

impl TodoState {
    pub fn to_i8(&self) -> i8 {
        match self {
            TodoState::Completed => 1,
            TodoState::NoStarted => 0,
        }
    }
}

impl ToString for TodoState {
    fn to_string(&self) -> String {
        match self {
            TodoState::Completed => String::from("Completed"),
            TodoState::NoStarted => String::from("Not started"),
        }
    }
}

#[derive(Debug)]
pub enum TodoError {
    ParseError,
    MissingData,
}

impl ToString for TodoError {
    fn to_string(&self) -> String {
        match self {
            TodoError::ParseError => "Cannot convert Todo".to_string(),
            TodoError::MissingData => "Missing data of Todo".to_string(),
        }
    }
}
