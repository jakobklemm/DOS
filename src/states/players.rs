use std::collections::HashMap;

use crate::error::TABError;

#[derive(PartialEq, Clone, Debug)]
pub struct Players (pub HashMap<String, Player>);

#[derive(PartialEq, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub distances: HashMap<Source, f64>,
}

impl Player {
    pub fn new(name: String, s: Source, d: f64) -> Self {
        let mut distances = HashMap::new();
        distances.insert(s, d);
        Self {
            name,
            distances
        }
    }

    pub fn parse<T>(line: T) -> Result<(String, f64), TABError> where T: Into<String> {
        let line: String = line.into();
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            return Err(TABError::default());
        }

        let name = parts.get(0).ok_or(TABError::default())?.to_string();
        let distance: f64 = parts.get(1).ok_or(TABError::default())?.parse::<f64>()?;

        Ok((name, distance))
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Source {
    pub id: String,
    pub position: Position,
}

impl Source {
    pub fn new() -> Self {
        Source {
            id: String::new(),
            position: Position { x: 0, y: 0, z: 0 }
        }
    }

    pub fn parse<T>(&mut self, line: T) -> Result<(), TABError> where T: Into<String> {
        let line: String = line.into();
        let mut parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 4 {
            return Err(TABError::default());
        }

        let id = parts.get(0).unwrap().to_string();

        let coords = parts.split_off(1);

        let position = Position::parse(coords)?;
        
        self.id = id;
        self.position = position;

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Position {
    x: i64,
    y: i64,
    z: i64
}

impl Position {
    fn parse(parts: Vec<&str>) -> Result<Self, TABError> {
        let x = parts.get(0).ok_or(TABError::default())?.parse::<i64>()?;
        let y = parts.get(1).ok_or(TABError::default())?.parse::<i64>()?;
        let z = parts.get(2).ok_or(TABError::default())?.parse::<i64>()?;
        Ok(Self {
            x,
            y,
            z
        })
    }
}

impl Players {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}