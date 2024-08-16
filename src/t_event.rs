use crate::t_file::TFile;


pub trait TEvent
{
    fn invoke(&mut self, t_file: &mut TFile);
    fn reverse(&self, t_file: &mut TFile);
}


pub struct InsertEvent(pub String);


impl TEvent for InsertEvent
{
    fn invoke(&mut self, t_file: &mut TFile)
    {
        t_file.insert(self.0.as_str()).unwrap();
    }

    fn reverse(&self, t_file: &mut TFile)
    {
        t_file.delete(self.0.len()).unwrap();
    }
}


pub struct DeleteEvent(pub usize, pub String);


impl TEvent for DeleteEvent
{
    fn invoke(&mut self, t_file: &mut TFile)
    {
        self.1 = t_file.delete(self.0).unwrap();
    }

    fn reverse(&self, t_file: &mut TFile)
    {
        t_file.insert(self.1.as_str()).unwrap();
    }
}


pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

pub struct MoveEvent(pub Direction, pub usize);


impl TEvent for MoveEvent
{
    fn invoke(&mut self, t_file: &mut TFile)
    {
        self.1 = t_file.index;
        match self.0
        {
            Direction::Up => { t_file.move_up().unwrap() },
            Direction::Down => { t_file.move_down().unwrap() },
            Direction::Left => { t_file.move_left(1).unwrap() },
            Direction::Right => { t_file.move_right(1).unwrap() },
        };
    }

    fn reverse(&self, t_file: &mut TFile)
    {
        t_file.move_to(self.1).unwrap();
    }
}