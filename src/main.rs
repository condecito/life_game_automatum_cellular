mod automaton;
use automaton::board;
use automaton::board::Board;
use automaton::cell::{Cell, Coordinates};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::Position;

#[allow(warnings, unused)]
#[allow(unused_variables)]
#[warn(unused_imports)]
#[allow(unused)]
fn main() {
    // default patterns
    let hive = [(12, 15), (12, 16), (13, 14), (13, 17), (14, 15), (14, 16)];
    let block = [(10, 10), (10, 11), (11, 10), (11, 11)];
    /*...##...
    ...##...*/

    let blinker = [(15, 15), (15, 16), (15, 17)];
    let glider_gun_coordinates = [
        (3, 9),
        (4, 9),
        (3, 10),
        (4, 10),
        (5, 7),
        (5, 8),
        (6, 6),
        (7, 6),
        (8, 6),
        (9, 7),
        (10, 8),
        (10, 9),
        (10, 10),
        (9, 10),
        (8, 10),
        (7, 9),
        (5, 18),
        (6, 18),
        (6, 19),
        (5, 19),
        (4, 21),
        (3, 21),
        (3, 23),
        (4, 23),
        (2, 22),
        (4, 22),
        (3, 26),
        (2, 26),
        (2, 27),
        (1, 27),
    ];
    let block2 = [(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)];
    // init board  width
    let mut board = Board::new(25, 100);
    // you can set differet patters to init the board also you can change the number of generations to wait for board
    board.start_board_xy(&block);
}
