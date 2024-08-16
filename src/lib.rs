use std::fs::{File};
use std::io::{stdout, Write, Seek, ErrorKind};
use crossterm::{
    execute,
    terminal::{Clear, ClearType, DisableLineWrap, EnableLineWrap},
    cursor::MoveTo,
    event::{Event, read, KeyCode, KeyEventKind, KeyModifiers},
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};

mod t_file;
use t_file::TFile;

mod t_event;
use t_event::{InsertEvent, DeleteEvent, Direction, MoveEvent};

mod t_file_explorer;
use t_file_explorer::TFileExplorer;


pub struct Config
{
    pub file_path: String,
    pub file_explorer: bool,
    pub create_file: bool,
    pub delete_file: bool,
    pub rename_file: bool,
    pub dark: bool,
    pub light: bool,
    pub new_file_name: String,
    pub help: bool,
    pub keybinds: bool,
}


const MODIFIERS: [&str; 10] = ["-c", "--create", "-d", "--delete", "-r", "--rename", "-b", "--dark", "-l", "--light"];


impl Config
{
    pub fn default() -> Config
    {
        Config {file_path: String::new(),
                file_explorer: false,
                create_file: false,
                delete_file: false,
                rename_file: false,
                dark: false,
                light: false,
                new_file_name: String::new(),
                help: false,
                keybinds: false,
            }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() <= 1
        {
            return Err("Not enough arguments");
        }

        if args.len() == 2
        {
            if args[1] == "--files" || args[1] == "-f"
            {
                let mut c = Config::default();
                c.file_explorer = true;
                return Ok(c);
            }
            if args[1] == "--help" || args[1] == "-h"
            {
                let mut c = Config::default();
                c.help = true;
                return Ok(c);
            }
            if args[1] == "--keybinds" || args[1] == "-k"
            {
                let mut c = Config::default();
                c.keybinds = true;
                return Ok(c);
            }

            let mut c = Config::default();
            c.file_path = args[1].clone();
            return Ok(c);
        }

        if args.len() == 3
        {
            if !MODIFIERS.contains(&args[2].as_str())
            {
                return Err("Invalid modifier");
            }

            let file_path = args[1].clone();

            if MODIFIERS[0..2].contains(&args[2].as_str())
            {
                let mut c = Config::default();
                c.file_path = file_path;
                c.create_file = true;
                return Ok(c);
            }
            if MODIFIERS[2..4].contains(&args[2].as_str())
            {
                let mut c = Config::default();
                c.file_path = file_path;
                c.delete_file = true;
                return Ok(c);
            }
            if MODIFIERS[4..6].contains(&args[2].as_str())
            {
                let mut c = Config::default();
                c.file_path = file_path;
                c.rename_file = true;
                return Ok(c);
            }
            if MODIFIERS[6..8].contains(&args[2].as_str())
            {
                let mut c = Config::default();
                c.file_path = file_path;
                c.dark = true;
                return Ok(c);
            }

            let mut c = Config::default();
            c.file_path = file_path;
            c.light = true;
            return Ok(c);
        }

        if args.len() == 4
        {
            if !MODIFIERS[4..6].contains(&args[2].as_str())
            {
                return Err("Too many arguments");
            }

            let mut c = Config::default();
            c.file_path = args[1].clone();
            c.new_file_name = args[3].clone();
            c.create_file = true;
            return Ok(c);
        }

        Err("Too many arguments")
    }
}


pub fn run(config: Config) -> Result<(), &'static str>
{
    if config.help
    {
        print!(r#"Command line text editor like vim. But tim.

Usage: tim <FILE_PATH> [OPTIONS]

Options:
    -c, --create        Creates but doesn't open file
    -d, --delete        Deletes file
    -r, --rename [NAME] Renames file to [NAME] or user inputted
    -b, --dark          White on black
    -l, --light         Black on white

Usage: tim [OPTIONS]

Options:
    -f, --files         Opens a file explorer to pick a file to open
    -h, --help          Shows commands
    -k, --keybinds      Shows keybinds/controls

"#);
        Ok(())
    }
    else if config.keybinds
    {
        print!(r#"Text Editor:
    Esc, End, Delete, Ctrl-S => Exit
    Arrow Keys => Move Cursor
    Ctrl-Z => Undo

File Explorer:
    Esc, End, Delete, Ctrl-S => Exit
    Arrow Keys => Move Cursor
    Enter, Space => Select
    Backspace => Parent Directory

"#);
        Ok(())
    }
    else if config.file_explorer
    {
        file_explorer()
    }
    else if config.create_file
    {
        create_file(config.file_path.as_str())
    }
    else if config.delete_file
    {
        delete_file(config.file_path.as_str())
    }
    else if config.rename_file
    {
        rename_file(config.file_path.as_str(), config.new_file_name)
    }
    else
    {
        if config.dark
        {
            execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::Black),
            ).unwrap();
        }
        else if config.light
        {
            execute!(
                stdout(),
                SetForegroundColor(Color::Black),
                SetBackgroundColor(Color::White),
            ).unwrap();
        }
        text_editor(config.file_path.as_str())
    }
}


fn create_file(path: &str) -> Result<(), &'static str>
{
    match File::create_new(path)
    {
        Ok(_) => Ok(()),
        Err(err) => {
            match err.kind()
            {
                ErrorKind::AlreadyExists => Err("File already exists."),
                _ => Err("Cannot create file."),
            }
        },
    }
}


fn delete_file(path: &str) -> Result<(), &'static str>
{
    match std::fs::remove_file(path)
    {
        Ok(_) => Ok(()),
        Err(err) => {
            match err.kind()
            {
                ErrorKind::NotFound => Err("File doesn't exist."),
                _ => Err("Cannot delete file."),
            }
        },
    }
}


fn rename_file(path: &str, mut name: String) -> Result<(), &'static str>
{
    if name.is_empty()
    {
        println!("Enter a new name for the file:");

        name = String::new();
        match std::io::stdin().read_line(&mut name)
        {
            Ok(_) => {},
            Err(_) => {
                return Err("Error reading name input.");
            },
        }
        println!();
    }

    match std::fs::rename(path, name.trim())
    {
        Ok(_) => Ok(()),
        Err(err) => {
            match err.kind()
            {
                ErrorKind::NotFound => Err("File doesn't exist."),
                _ => { println!("{}", err); Err("Cannot rename file.") },
            }
        },
    }
}


fn file_explorer() -> Result<(), &'static str>
{
    let mut t_file_explorer = TFileExplorer::new();
    let selected_path: &str;

    crossterm::terminal::enable_raw_mode().unwrap();

    execute!(
        stdout(),
        DisableLineWrap,
    ).unwrap();

    t_file_explorer.clear_screen().unwrap();

    loop
    {
        match read().unwrap()
        {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press
                {
                    match event.code
                    {
                        KeyCode::Esc | KeyCode::End | KeyCode::Delete => { selected_path = ""; break; },
                        KeyCode::Char('s') =>
                            {
                                if event.modifiers == KeyModifiers::CONTROL
                                {
                                    selected_path = "";
                                    break;
                                }
                            },

                        KeyCode::Up => { t_file_explorer.move_up().unwrap(); },
                        KeyCode::Down => { t_file_explorer.move_down().unwrap(); },
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            match t_file_explorer.select()
                            {
                                None => { t_file_explorer.make_paths(); },
                                Some(path) => { selected_path = path; break; },
                            };
                        },
                        KeyCode::Backspace => { t_file_explorer.back().unwrap(); },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    execute!(
        stdout(),
        EnableLineWrap,
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        MoveTo(0, 0),
    ).unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();

    if selected_path.is_empty()
    {
        Ok(())
    }
    else
    {
        text_editor(selected_path)
    }
}


fn text_editor(path: &str) -> Result<(), &'static str>
{
    let mut t_file;
    match open_file(path)
    {
        Ok(f) => t_file = f,
        Err(err) => return Err(err),
    }

    crossterm::terminal::enable_raw_mode().unwrap();

    t_file.clear_screen().unwrap();

    loop
    {
        match read().unwrap()
        {
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press
                {
                    match event.code
                    {
                        KeyCode::Esc | KeyCode::End | KeyCode::Delete => break,
                        KeyCode::Char('s') =>
                            {
                                if event.modifiers == KeyModifiers::CONTROL
                                {
                                    break;
                                }
                                t_file.add_event(InsertEvent(event.code.to_string()));
                            },

                        KeyCode::Up => { t_file.add_event(MoveEvent(Direction::Up, 0)); },
                        KeyCode::Down => { t_file.add_event(MoveEvent(Direction::Down, 0)); },
                        KeyCode::Left => { t_file.add_event(MoveEvent(Direction::Left, 0)); },
                        KeyCode::Right => { t_file.add_event(MoveEvent(Direction::Right, 0)); },

                        KeyCode::Enter => { t_file.add_event(InsertEvent(String::from("\n"))); },
                        KeyCode::Backspace => { t_file.add_event(DeleteEvent(1, String::new())); },

                        KeyCode::Char('z') =>
                            {
                                if event.modifiers == KeyModifiers::CONTROL
                                {
                                    t_file.undo();
                                }
                                else
                                {
                                    t_file.add_event(InsertEvent(event.code.to_string()));
                                }
                            },

                        KeyCode::Tab => { t_file.add_event(InsertEvent(String::from("    "))); },
                        KeyCode::Char(' ') => { t_file.add_event(InsertEvent(String::from(" "))); },
                        _ => {
                            if event.code.to_string().len() == 1
                            {
                                t_file.add_event(InsertEvent(event.code.to_string()));
                            }
                        },
                    }
                }
            },
            _ => {},
        }
    }

    execute!(
        stdout(),
        ResetColor,
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        MoveTo(0, 0),
    ).unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();

    t_file.file.set_len(0).unwrap();
    t_file.file.rewind().unwrap();
    t_file.file.write(t_file.content.as_ref()).unwrap();

    Ok(())
}


fn open_file(file_path: &str) -> Result<TFile, &'static str>
{
    match File::options().write(true).read(true).create(true).open(file_path)
    {
        Ok(f) => match TFile::build(f) {
            Ok(t_file) => Ok(t_file),
            Err(err) => Err(err),
        },
        Err(err) => {
            match err.kind()
            {
                _ => Err("File cannot be opened"),
            }
        },
    }
}
