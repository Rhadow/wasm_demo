use super::Cell;
use super::character::Character;
use super::constants::{
    WORLD_WIDTH_IN_CELLS,
    WORLD_HEIGHT_IN_CELLS
};

pub struct World {
    width_in_cells: usize,
    height_in_cells: usize,
    cells: Vec<Cell>,
    characters: Vec<Box<dyn Character>>,
}

impl World {
    pub fn new() -> World {
        World {
            width_in_cells: WORLD_WIDTH_IN_CELLS,
            height_in_cells: WORLD_HEIGHT_IN_CELLS,
            cells: vec![],
            characters: vec![],
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        self.width_in_cells * row + col
    }

    pub fn width_in_cells(&self) -> usize {
        self.width_in_cells
    }

    pub fn height_in_cells(&self) -> usize {
        self.height_in_cells
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn set_characters(&mut self, characters: Vec<Box<dyn Character>>) {
        self.characters = characters;
    }

    pub fn get_characters(&self) -> &Vec<Box<dyn Character>> {
        &self.characters
    }

    pub fn update(&mut self) {
        for character in &mut self.characters {
            character.update();
        }
    }
}