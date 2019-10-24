use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;

use crate::Color;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PainterParams {
    pub painter: String,
    pub global_brightness: f32,
    pub speed: f32,
    pub color: Color,
    pub secondary_colors: Vec<Color>,
    pub fade: f32,
    pub bidirectional: bool,
}

impl PainterParams {
    pub fn serialize(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
    pub fn deserialize(string: &str) -> Result<Self, Box<dyn Error>> {
        let p: PainterParams = serde_json::from_str(string)?;
        return Ok(p);
    }
    pub fn apply_dimming(&mut self) {
        self.color *= self.global_brightness;
        for idx in 0..self.secondary_colors.len() {
            self.secondary_colors[idx] *= self.global_brightness;
        }
    }
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut file = File::open("last_params.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Self::deserialize(&contents);
    }
    pub fn save(&self) -> std::io::Result<()> {
        let contents = self.serialize();
        let mut file = File::create("last_params.json")?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}
