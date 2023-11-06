use std::fmt::Display;

use crossterm::style::{StyledContent, Stylize};

pub struct Item<T: Display>(T);

impl<T> Item<T>
where
    T: Display,
{
    pub fn new(content: T) -> Self {
        Self(content)
    }

    pub fn print(&self) -> StyledContent<String> {
        self.0.to_string().dark_grey()
    }

    pub fn print_active(&self) -> StyledContent<String> {
        format!("> {}", self.0).blue().bold()
    }

    pub fn get_raw_item(&self) -> &T {
        &self.0
    }
}

impl<T> Display for Item<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
