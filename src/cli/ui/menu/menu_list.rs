use std::{
    fmt::Display,
    io::{Stdout, Write},
};

use crossterm::{cursor, queue, style, terminal};

use super::Item;

pub struct Menu<T: Display> {
    items: Vec<Item<T>>,
    current: usize,
    offset_top: u16,
    padding: u16,
}

impl<T> Menu<T>
where
    T: Display,
{
    pub fn new(items: Vec<Item<T>>) -> anyhow::Result<Self> {
        Ok(Self {
            items,
            current: 0,
            offset_top: cursor::position()?.1,
            padding: 2,
        })
    }

    pub fn display(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for idx in 0..self.items.len() {
            self.draw_item(stdout, idx)?;
        }

        stdout.flush()?;

        Ok(())
    }

    fn draw_item(&self, stdout: &mut Stdout, idx: usize) -> anyhow::Result<()> {
        let move_cursor = cursor::MoveTo(
            if self.current == idx {
                self.padding - 2
            } else {
                self.padding
            },
            idx as u16 + self.offset_top,
        );
        let clear_line = terminal::Clear(terminal::ClearType::CurrentLine);

        let item = self
            .items
            .get(idx)
            .ok_or(anyhow::anyhow!("line index out of bounds: {idx}"))?;

        queue!(
            stdout,
            move_cursor,
            clear_line,
            style::PrintStyledContent(if self.current == idx {
                item.print_active()
            } else {
                item.print()
            })
        )?;

        Ok(())
    }

    pub fn hide(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for line in 0..self.items.len() {
            queue!(
                stdout,
                cursor::MoveTo(self.padding, self.offset_top + line as u16 + 1),
                terminal::Clear(terminal::ClearType::CurrentLine)
            )?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn next_item(&mut self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let prev_active = self.current;

        self.current = if self.current == self.items.len() - 1 {
            0
        } else {
            self.current + 1
        };

        self.draw_item(stdout, self.current)?;
        self.draw_item(stdout, prev_active)?;
        stdout.flush()?;

        Ok(())
    }

    pub fn prev_item(&mut self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let prev_active = self.current;

        self.current = if self.current == 0 {
            self.items.len() - 1
        } else {
            self.current - 1
        };

        self.draw_item(stdout, self.current)?;
        self.draw_item(stdout, prev_active)?;
        stdout.flush()?;

        Ok(())
    }

    pub fn get_current_item(&self) -> &Item<T> {
        &self.items[self.current]
    }
}

#[derive(Default)]
pub struct MenuBuilder<T: Display> {
    items: Vec<Item<T>>,
    offset_top: u16,
    padding: u16,
}

impl<T> MenuBuilder<T>
where
    T: Display,
{
    pub fn build(self) -> Menu<T> {
        Menu {
            items: self.items,
            current: 0,
            offset_top: self.offset_top,
            padding: self.padding,
        }
    }

    pub fn items(self, items: Vec<Item<T>>) -> Self {
        Self { items, ..self }
    }

    pub fn offset_top(self, offset_top: u16) -> Self {
        Self { offset_top, ..self }
    }

    pub fn padding(self, padding: u16) -> Self {
        Self { padding, ..self }
    }
}

impl<T> Menu<T>
where
    T: Display + Default,
{
    pub fn builder() -> MenuBuilder<T> {
        MenuBuilder::default()
    }
}
