#[allow(warnings, unused)]
#[allow(unused_variables)]
#[warn(unused_imports)]
#[allow(unused)]
use super::cell::{Cell, CellStatus, Coordinates};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::Print;
use crossterm::{queue, terminal, ExecutableCommand};
use graphics::ellipse;
use std::collections::{HashMap, HashSet};
use std::fmt::{format, Error};
use std::io::Write;
use std::ops::Index;
use std::sync::Mutex;
use std::time::Duration;
use std::{cell, thread, vec};
use std::{
    io::{stdout, Stdout},
    process::Command,
};

// TODO: you can play with this value and take a look how many generations the cell keep alive
const NUMBER_GENERATIONS: usize = 25;
#[derive(Clone)]
pub struct Board {
    buffer: Vec<Cell>,
    width: i32,
    height: i32,
}
// board
impl Board {
    pub fn new(with: i32, height: i32) -> Self {
        let mut buffer: Vec<Cell> = Vec::new();
        // conver the matriz to vector
        for x in 0..with {
            for y in 0..height {
                let cell = Cell::new(Coordinates::new(x, y));
                //   print!("{:?}\n", cell);
                buffer.push(cell);
            }
        }

        Board {
            buffer,
            width: with,
            height,
        }
    }

    // start a board with speciic position
    pub fn start_board_xy(&mut self, h: &[(i32, i32)]) {
        for v in h {
            let pos = self.get_position_on_vector(v.0 - 1, v.1 - 1).unwrap();
            self.buffer[pos].change_state(CellStatus::Alive);
        }

        let mut std = stdout();
        queue!(std, terminal::Clear(terminal::ClearType::All)).unwrap();

        for i in 0..=NUMBER_GENERATIONS {
            let mut vec_copy: Vec<Cell> = self.buffer.clone();

            for (index, cell) in self.buffer.iter().enumerate() {
                if *cell.get_status() == CellStatus::Alive {
                    self.print_console(
                        cell.get_coordinates().x(),
                        cell.get_coordinates().y(),
                        &mut std,
                        '#',
                    );
                } else {
                    self.print_console(
                        cell.get_coordinates().x(),
                        cell.get_coordinates().y(),
                        &mut std,
                        '.',
                    );
                }
                let neighbors_index = self.get_neighbors_from_position(
                    cell.get_coordinates().x(),
                    cell.get_coordinates().y(),
                );
                let mut alive_neighbors = 0;
                for i in neighbors_index {
                    if *self.buffer[i].get_status() == CellStatus::Alive {
                        alive_neighbors += 1;
                    }
                }

                let mut new_cell = *cell;
                if *cell.get_status() == CellStatus::Alive {
                    //rule 1,4
                    /*1.Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    4.Any live cell with more than three live neighbours dies, as if by overpopulation.
                    */
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_cell = Cell::new_from_status(*cell.get_coordinates(), CellStatus::Dead);
                    } else {
                        if alive_neighbors == 2 || alive_neighbors == 3 {
                            //rule 3.Any live cell with two or three live neighbours lives on to the next generation.
                            new_cell =
                                Cell::new_from_status(*cell.get_coordinates(), CellStatus::Alive);
                        }
                    }
                } else {
                    //rule 2.Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    if alive_neighbors == 3 {
                        new_cell =
                            Cell::new_from_status(*cell.get_coordinates(), CellStatus::Alive);
                    }
                }
                vec_copy[index] = new_cell;
            }
            self.buffer = vec_copy;
            print!("\n\ngeneration {}\n", i);
            thread::sleep(Duration::from_millis(1000));
            // print!("------------------------------------------------------\n");
        }
    }

    // retunrs hashSet with all neighbors from cell position
    fn get_neighbors_from_position(&self, x: i32, y: i32) -> HashSet<usize> {
        if x > self.width || y > self.height {
            panic!("Invalid Coordinates\n");
        }
        let mut neighbors: HashSet<usize> = HashSet::new();
        let mut x_mod = x - 1;
        let mut y_mod = y - 1;

        for a in -1..=1 {
            for b in -1..=1 {
                let mut x_mod = x_mod + a;
                let mut y_mod = y_mod + b;

                if x_mod < 0 || y_mod < 0 {
                    continue;
                }
                if x_mod >= self.width || y_mod >= self.height {
                    continue;
                }
                if x_mod == x - 1 && y_mod == y - 1 {
                    continue;
                }

                //  print!("element x= {},y= {}\n", x_mod, y_mod);
                let exist = self.get_position_on_vector(x_mod, y_mod);
                match exist {
                    Ok(index) => {
                        neighbors.insert(index);
                    }
                    _ => {}
                }
            }
        }

        neighbors
    }

    // Returns the position of a cell within on Vector
    fn get_position_on_vector(&self, x: i32, y: i32) -> Result<usize, i32> {
        // get the position from matriz on vector

        let mut index = x as usize * self.height as usize + y as usize; //position on matriz

        if index < self.buffer.len() {
            //  print!("valid element ({},{}) index on vector={}\n", x, y, index);
            return Ok(index);
        } else {
            return Err(-1);
        }
    }

    fn print_console(&self, x: i32, y: i32, stdout: &mut Stdout, letter: char) {
        stdout.execute(Hide).unwrap();
        queue!(stdout, MoveTo(y as u16, x as u16)).unwrap();
        queue!(stdout, Print(letter)).unwrap();
        stdout.flush().unwrap();
    }

    pub fn get_board(self) -> Vec<Cell> {
        self.buffer
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use crate::automaton::cell::Coordinates;
    #[test]
    fn test_neighbors() {
        let mut board = Board::new(3, 3);
        //  let genesis = Coordinates::new(5, 5);
        let neighbors = board.get_neighbors_from_position(1, 1);
        let cells = board.get_board();
        print!("neighbords number={}\n", neighbors.len());
        for element in neighbors {
            print!("Cell= {:?}, index={}\n", cells[element], element);
        }
    }
}
