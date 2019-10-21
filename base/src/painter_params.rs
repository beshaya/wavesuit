use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::Color;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PainterParams {
    pub painter: String,
    pub global_brightness: f32,
    pub speed: f32,
    pub color: Color,
    pub secondary_colors: Vec<Color>,
}

impl PainterParams {
    pub fn serialize(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
    pub fn deserialize(string: &str) -> Result<PainterParams, Box<dyn Error>> {
        let p: PainterParams = serde_json::from_str(string)?;
        return Ok(p);
    }
    pub fn apply_dimming(&mut self) {
        self.color *= self.global_brightness;
        for idx in 0..self.secondary_colors.len() {
            self.secondary_colors[idx] *= self.global_brightness;
        }
    }
}
