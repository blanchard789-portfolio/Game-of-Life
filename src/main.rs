#![no_std]
#![no_main]

mod life;
use life::*;

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use nanorand::{Pcg64, Rng, SeedableRng};

/*
enum State {
    LedOn,
    LedOff,
}
*/

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = nanorand::Pcg64::new_seed(1);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut game_board: [[u8; 5]; 5] = [[0; 5]; 5];
    random_board(&mut rng, &mut game_board);
    let mut b_frame_pause = 0;

    let mut end_game_check = false;
    let mut no_pixel_pause = 0;

    loop {
        if button_a.is_low().unwrap() {
            rprintln!("a button");
            random_board(&mut rng, &mut game_board);
        } else if button_b.is_low().unwrap() && b_frame_pause == 0 {
            rprintln!("b button");
            flip_pixels(&mut game_board);
            b_frame_pause = 5;
        }

        if b_frame_pause > 0 {
            b_frame_pause -= 1;
        }

        if done(&game_board) && !end_game_check {
            end_game_check = true;
            no_pixel_pause = 5;
        } else if done(&game_board) && no_pixel_pause > 0 {
            no_pixel_pause -= 1;
        } else if done(&game_board) && no_pixel_pause == 0 {
            end_game_check = false;
            no_pixel_pause = 0;
            random_board(&mut rng, &mut game_board);
        } else {
            end_game_check = false;
            no_pixel_pause = 0;
        }

        life(&mut game_board);
        display.show(&mut timer, game_board, 100);
    }
}

// fn random_board -> [[u8; 5]; 5]
// Generates a random 5x5 matrix of u8.
// Used for creating new game boards.
fn random_board(rng: &mut Pcg64, rng_board: &mut [[u8; 5]; 5]) {
    #[allow(clippy::needless_range_loop)]
    for row in 0..5 {
        for col in 0..5 {
            let b: bool = rng.generate();
            rng_board[row][col] = b as u8;
        }
    }
}

// fn flip_pixels -> [[u8; 5]; 5]
// Flips all pixels in 5x5 matrix.
fn flip_pixels(board_in: &mut [[u8; 5]; 5]) {
    #[allow(clippy::needless_range_loop)]
    for row in 0..5 {
        for col in 0..5 {
            if board_in[row][col] == 1 {
                board_in[row][col] = 0;
            } else {
                board_in[row][col] = 1;
            }
        }
    }
}
