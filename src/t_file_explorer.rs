use std::fs::read_dir;
use std::io::stdout;
use std::path::{Path, PathBuf};
use crossterm::{
    execute, ExecutableCommand,
    cursor::{MoveTo, RestorePosition, SavePosition, position, MoveDown, MoveUp},
    terminal::{Clear, ClearType, size},
};


pub struct TFileExplorer
{
    pub directory: String,
    pub paths: Vec<PathBuf>,
    pub view: [usize; 2],
}


impl TFileExplorer
{
    pub fn new() -> TFileExplorer
    {
        let mut t_file_explorer = TFileExplorer { directory: String::from("./"), paths: Vec::new(), view: [0, 0] };
        t_file_explorer.make_paths();
        t_file_explorer
    }

    pub fn select(&mut self) -> Option<&str>
    {
        let (_, y) = position().unwrap();

        let path = self.paths.get((y - 1) as usize)?.as_path();

        if path.is_dir()
        {
            self.directory = String::from(path.to_str()?);
            return None;
        }
        if !path.exists() || !path.is_absolute() || !path.is_file()
        {
            return None;
        }

        Some(path.to_str()?)
    }

    pub fn back(&mut self) -> std::io::Result<()>
    {
        let path: &Path = match Path::new(self.directory.as_str()).parent()
        {
            None => return Ok(()),
            Some(p) => p,
        };
        self.directory = path.to_str().unwrap().to_string();

        self.make_paths();

        Ok(())
    }

    pub fn move_up(&mut self) -> std::io::Result<()>
    {
        let (_, y) = position()?;

        if y > 1
        {
            execute!(
                stdout(),
                MoveUp(1),
                SavePosition,
            )?;
        }
        else if self.view[0] > 0
        {
            self.view[0] -= 1;
            self.view[1] -= 1;
            self.clear_screen()?;
        }

        Ok(())
    }

    pub fn move_down(&mut self) -> std::io::Result<()>
    {
        let (_, rows) = size()?;
        let (_, y) = position()?;

        if y < self.paths.len() as u16
        {
            if y < rows - 1
            {
                execute!(
                    stdout(),
                    MoveDown(1),
                    SavePosition,
                )?;
            }
            else if self.view[1] < self.paths.len()
            {
                self.view[0] += 1;
                self.view[1] += 1;
                self.clear_screen()?;
            }
        }

        Ok(())
    }

    pub fn clear_screen(&mut self) -> std::io::Result<()>
    {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Clear(ClearType::Purge),
            MoveTo(0, 0),
        )?;

        if self.paths.is_empty()
        {
            println!("{}\n└──Empty Folder", Path::new(self.directory.as_str()).display());

            stdout().execute(RestorePosition)?;

            return Ok(());
        }

        println!("{}", Path::new(self.directory.as_str()).display());

        for (i, path) in self.paths[self.view[0]..self.view[1]].iter().enumerate()
        {
            let path = path.file_name().unwrap().to_str().unwrap();
            if i + self.view[0] < self.view[1] - 1
            {
                println!("├──{path}");
            }
            else
            {
                if self.view[1] == self.paths.len()
                {
                    print!("└──{path}");
                }
                else
                {
                    print!("├──{path}");
                }
            }
        }

        stdout().execute(RestorePosition)?;

        Ok(())
    }

    pub fn make_paths(&mut self)
    {
        let dir = match read_dir(Path::new(self.directory.as_str()))
        {
            Ok(d) => d,
            Err(_) =>
            {
                self.directory = Path::new(self.directory.as_str()).parent().unwrap().to_str().unwrap().to_string();
                return;
            },
        };

        let _ = execute!(
            stdout(),
            MoveTo(3, 1),
            SavePosition,
        );

        self.paths.clear();

        for path in dir
        {
            let s_path = path.iter().clone().next().unwrap();
            match s_path.path().canonicalize()
            {
                Ok(p) => {
                    let path = &p.to_str().unwrap()[4..];
                    self.paths.push(PathBuf::from(path));
                    continue;
                },
                Err(_) => {},
            };
            self.paths.push(path.unwrap().path());
        }

        match self.paths.get(0)
        {
            None => {},
            Some(p) => self.directory = p.parent().unwrap().to_str().unwrap().to_string(),
        }

        self.view[1] = self.paths.len();
        let (_, rows) = size().unwrap();
        self.view[1] = self.view[1].clamp(0, rows as usize - 1);

        self.clear_screen().unwrap();
    }
}