use std::collections::HashMap;

use serde::Serialize;

use std::hash::{Hash, Hasher};

use crate::error::TABError;

#[derive(PartialEq, Clone, Debug)]
pub struct Players(pub HashMap<String, Player>);

#[derive(PartialEq, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub distances: HashMap<Source, f64>,
}

impl Player {
    pub fn new(name: String, s: Source, d: f64) -> Self {
        let mut distances = HashMap::new();
        distances.insert(s, d);
        Self { name, distances }
    }

    pub fn parse<T>(line: T) -> Result<(String, f64), TABError>
    where
        T: Into<String>,
    {
        let line: String = line.into();
        let parts: Vec<&str> = line.split(":").collect();

        if parts.len() != 2 {
            return Err(TABError::default());
        }

        let name = parts.get(0).ok_or(TABError::default())?.to_string();
        let distance: f64 = parts.get(1).ok_or(TABError::default())?.parse::<f64>()?;

        Ok((name, distance))
    }

    pub fn position(&self) -> Result<Position, TABError> {
        if self.distances.len() != 3 {
            return Err(TABError::default());
        }

        let mut p1 = Source::new();
        let mut p2 = Source::new();
        let mut p3 = Source::new();

        let mut rs: [f64; 16] = [0.0; 16];

        println!("info: {:?}", self);

        // let sources: Vec<Source> = self.distances.into_values().collect();

        for (i, (source, distance)) in self.distances.iter().enumerate() {
            rs[i] = *distance;

            println!("Distance {}: {}", i, *distance);

            match i {
                0 => {
                    p1 = source.clone();
                }
                1 => {
                    p2 = source.clone();
                }
                2 => {
                    p3 = source.clone();
                }
                _ => break,
            }
        }

        let d_x = (p2.position.x - p1.position.x).pow(2) as f64;
        let d_y = (p2.position.z - p1.position.z).pow(2) as f64;
        let d = (d_x + d_y).sqrt();

        let a = (rs[0].powi(2) - rs[1].powi(2) + d.powi(2)) / (2.0 * d);
        let h = (rs[0].powi(2) - a.powi(2)).sqrt();

        println!("Values: d: {}, a: {}, h: {}", d, a, h);

        let mid_x = p1.position.x as f64 + a * ((p2.position.x as f64 - p1.position.x as f64) / d);
        let mid_z = p1.position.z as f64 + a * ((p2.position.z as f64 - p1.position.z as f64) / d);

        let p_x_1 = mid_x + h * (p2.position.z as f64 - p1.position.z as f64) / d;
        let p_z_1 = mid_z + h * (p2.position.x as f64 - p1.position.x as f64) / d;

        let p_x_2 = mid_x - h * (p2.position.z as f64 - p1.position.z as f64) / d;
        let p_z_2 = mid_z - h * (p2.position.x as f64 - p1.position.x as f64) / d;

        println!("Point 1: ({}, {})", p_x_1, p_z_1);
        println!("Point 2: ({}, {})", p_x_2, p_z_2);

        let dist_to_x_1 = (p_x_1 + p3.position.x as f64).powi(2);
        let dist_to_z_1 = (p_z_1 + p3.position.z as f64).powi(2);

        let dist_1 = (dist_to_x_1 + dist_to_z_1).sqrt();

        let dist_to_x_2 = (p_x_2 + p3.position.x as f64).powi(2);
        let dist_to_z_2 = (p_z_2 + p3.position.z as f64).powi(2);

        let dist_2 = (dist_to_x_2 + dist_to_z_2).sqrt();

        println!("Distance 1: {}, Distance 2: {}", dist_1, dist_2);

        let diff_1 = (rs[2] - dist_1).abs();
        let diff_2 = (rs[2] - dist_2).abs();

        if diff_1 < diff_2 {
            let p = Position {
                x: p_x_1 as i64,
                y: 0,
                z: p_z_1 as i64,
            };
            return Ok(p);
        } else {
            let p = Position {
                x: p_x_2 as i64,
                y: 0,
                z: p_z_2 as i64,
            };
            return Ok(p);
        }
    }
}
impl ToString for Position {
    fn to_string(&self) -> String {
        return format!("{}, {}, {}", self.x, self.y, self.z);
    }
}

#[derive(Eq, Clone, Debug)]
pub struct Source {
    pub id: String,
    pub position: Position,
}

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Source {
    pub fn new() -> Self {
        Source {
            id: String::new(),
            position: Position { x: 0, y: 0, z: 0 },
        }
    }

    pub fn parse<T>(&mut self, line: T) -> Result<(), TABError>
    where
        T: Into<String>,
    {
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
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    fn parse(parts: Vec<&str>) -> Result<Self, TABError> {
        let x = parts.get(0).ok_or(TABError::default())?.parse::<f64>()?;
        let y = parts.get(1).ok_or(TABError::default())?.parse::<f64>()?;
        let z = parts.get(2).ok_or(TABError::default())?.parse::<f64>()?;
        Ok(Self { x: x as i64, y: y as i64, z: z as i64 })
    }
}

impl Players {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
