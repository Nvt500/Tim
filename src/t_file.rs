use crossterm::{
    cursor::{MoveTo, position},
    terminal::{size, Clear, ClearType},
    ExecutableCommand, execute,
};
use std::fs::File;
use std::io::{stdout, Read};

use crate::t_event::{TEvent};


pub struct TFile
{
    pub file: File,
    pub content: String,
    pub index: usize,
    pub view: [usize; 2],
    pub lines: Vec<String>,
    pub event_buffer: Vec<Box<dyn TEvent>>,
}


impl TFile
{
    pub fn build(file: File) -> Result<TFile, &'static str>
    {
        let (_, rows) = size().unwrap();
        let mut t_file = TFile { file,
            content: String::new(),
            index: 0,
            view: [0, rows as usize],
            lines: Vec::new(),
            event_buffer: Vec::new()
        };

        match t_file.file.read_to_string(&mut t_file.content)
        {
            Ok(_) => Ok(t_file),
            Err(_) => Err("Problem reading file."),
        }
    }

    pub fn add_event(&mut self, mut t_event: impl TEvent + 'static)
    {
        t_event.invoke(self);
        self.event_buffer.push(Box::new(t_event));

        if self.event_buffer.len() > 10
        {
            self.event_buffer.remove(0);
        }
    }

    pub fn undo(&mut self)
    {
        if self.event_buffer.len() > 0
        {
            self.event_buffer.pop().unwrap().reverse(self);
        }
    }

    pub fn insert(&mut self, string: &str) -> std::io::Result<()>
    {
        self.content.insert_str(self.index, string);
        self.index += string.len();

        self.clear_screen()?;

        Ok(())
    }

    pub fn delete(&mut self, units: usize) -> std::io::Result<String>
    {
        let mut string = String::new();
        for _ in 0..units
        {
            if self.content.len() > 0
            {
                if self.index > 0
                {
                    string.push(self.content.remove(self.index - 1));
                    self.index -= 1;
                }
                else
                {
                    string.push(self.content.remove(self.index));
                }
                self.clear_screen()?;
            }
        }

        Ok(string)
    }

    pub fn move_up(&mut self) -> std::io::Result<()>
    {
        let mut line_index: usize = 0;
        for (i, line) in self.lines.iter().enumerate()
        {
            if line_index + line.len() > self.index
            {
                if i == 0
                {
                    self.index = 0;
                    break;
                }

                let index = self.index - line_index;

                self.index -= line[..index + 1].len();

                if self.lines.get(i - 1).unwrap().len() > index
                {
                    self.index -= self.lines.get(i - 1).unwrap().len() - index - 1;
                }

                break;
            }
            else if line_index + line.len() == self.index && i == self.lines.len() - 1
            {
                self.index -= self.lines.get(i).unwrap().len();
            }
            line_index += line.len();
        }

        self.view_changed()?;

        Ok(())
    }

    pub fn move_down(&mut self) -> std::io::Result<()>
    {
        let mut line_index: usize = 0;
        for (i, line) in self.lines.iter().enumerate()
        {
            if i == self.lines.len() - 1
            {
                self.index = self.content.len();
                break;
            }

            if line_index + line.len() > self.index
            {
                let index = self.index - line_index;

                self.index += line[index..].len();

                if index >= self.lines.get(i + 1).unwrap().len()
                {
                    self.index += self.lines.get(i + 1).unwrap().len() - 1;
                }
                else
                {
                    self.index += index;
                }

                break;
            }

            line_index += line.len();
        }

        self.view_changed()?;

        Ok(())
    }

    pub fn move_left(&mut self, units: usize) -> std::io::Result<()>
    {
        if self.index > units - 1
        {
            self.index -= units;
            self.view_changed()?;
        }

        Ok(())
    }

    pub fn move_right(&mut self, units: usize) -> std::io::Result<()>
    {
        if self.index < self.content.len() + 1 - units
        {
            self.index += units;
            self.view_changed()?;
        }

        Ok(())
    }

    pub fn move_to(&mut self, index: usize) -> std::io::Result<()>
    {
        self.index = index.clamp(0, self.content.len());

        self.view_changed()?;

        Ok(())
    }

    pub fn clear_screen(&mut self) -> std::io::Result<()>
    {
        self.make_lines().unwrap();

        self.make_view()?;

        execute!(
            stdout(),
            Clear(ClearType::All),
            Clear(ClearType::Purge),
            MoveTo(0, 0),
        )?;

        let (_, rows) = size()?;
        if self.lines.len() > rows as usize
        {
            for (i, line) in self.lines[self.view[0]..self.view[1]].iter().enumerate()
            {
                if i < (rows - 1) as usize
                {
                    println!("{}", line.trim());
                }
                else
                {
                    print!("{}", line.trim());
                }
            }
        }
        else
        {
            print!("{}", self.content);

            let mut print_string = String::new();

            let (_, y) = position()?;

            for i in y + 1..rows
            {
                if i == 0
                {
                    print_string.push_str("~");
                }
                else
                {
                    print_string.push_str("\n~");
                }
            }

            print!("{}", print_string);
        }

        self.move_cursor()?;

        Ok(())
    }

    fn move_cursor(&mut self) -> std::io::Result<()>
    {
        let (cols, _) = size()?;

        let mut cursor: [u16; 2] = [0, 0];
        for (i, c) in self.content.chars().enumerate()
        {
            if i < self.index
            {
                if c == '\n' || cursor[0] >= cols - 1
                {
                    cursor = [0, cursor[1] + 1];
                }
                else
                {
                    cursor[0] += 1;
                }
            } else { break; }
        }

        cursor[1] -= self.view[0] as u16;

        stdout().execute(MoveTo(cursor[0], cursor[1]))?;

        Ok(())
    }

    fn view_changed(&mut self) -> std::io::Result<()>
    {
        let v0 = self.view[0];
        self.make_view()?;
        if v0 != self.view[0]
        {
            self.clear_screen()?;
        }
        else
        {
            self.move_cursor()?;
        }
        Ok(())
    }

    fn make_view(&mut self) -> std::io::Result<()>
    {
        let (_, rows) = size()?;
        self.view[0] = 0;
        self.view[1] = rows as usize;

        let mut line_index: usize = 0;
        for (i, line) in self.lines.iter().enumerate()
        {
            if line_index + line.len() > self.index || i == self.lines.len() - 1
            {
                if i > rows as usize - 1
                {
                    self.view[0] += i - rows as usize + 1;
                    self.view[1] += i - rows as usize + 1;
                }
                break;
            }
            line_index += line.len();
        }

        Ok(())
    }

    fn make_lines(&mut self) -> Result<(), &'static str>
    {
        let (cols, _) = match size() {
            Ok(s) => s,
            Err(_) => return Err("Function \"size()\" failed."),
        };

        self.lines.clear();

        for mut line in self.content.split("\n")
        {
            loop
            {
                if line.len() <= cols as usize
                {
                    break;
                }

                let (s1, s2) = line.split_at(cols as usize);
                self.lines.push(s1.to_string());
                line = s2;
            }

            self.lines.push(format!("{line}\n"));
        }

        Ok(())
    }
}