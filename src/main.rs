mod day_one;
use day_one::*;

mod day_two;
use day_two::*;

mod day_three;
use day_three::*;

mod day_four;
use day_four::*;

mod day_five;
use day_five::*;

mod day_six;
use day_six::*;

mod day_seven;
use day_seven::*;

mod day_eight;
use day_eight::*;

mod day_nine;
use day_nine::*;

mod day_ten;
use day_ten::*;

mod day_eleven;
use day_eleven::*;

mod day_twelve;
use day_twelve::*;

mod day_thirteen;
use day_thirteen::*;

mod day_fourteen;
use day_fourteen::*;

mod day_fifteen;
use day_fifteen::*;

mod day_sixteen;
use day_sixteen::*;

mod day_seventeen;
use day_seventeen::*;

mod day_eighteen;
use day_eighteen::*;

mod day_nineteen;
use day_nineteen::*;

mod day_twenty;
use day_twenty::*;

mod day_twenty_one;
use day_twenty_one::*;

mod day_twenty_two;
use day_twenty_two::*;

mod day_twenty_three;
use day_twenty_three::*;

mod day_twenty_four;
use day_twenty_four::*;

mod day_twenty_five;
use day_twenty_five::*;

mod utils;

use std::env;

// Tiny wrappers to invoke both parts of a single day at once, when needed.
fn day_one_wrapper() {
    day_one(1);
    day_one(1007);
}

fn day_two_wrapper() {
    day_two();
    day_two_part_two();
}

fn day_three_wrapper() {
    day_three();
    day_three_part_two();
}

fn day_four_wrapper() {
    day_four();
    day_four_part_two();
}

fn day_five_wrapper() {
    day_five();
    day_five_part_two();
}

fn day_ten_wrapper() {
    day_ten();
    day_ten_part_2();
}

fn day_twenty_two_wrapper() {
    day_twenty_two();
    day_twenty_two_part_two();
}

// Defining a new binary project for each Advent day was overkill; so we have one main
// taking an integer on the command line to decide which day to execute. The main panics
// if the parameter is invalid or non-existent. Once a day is selected we always execute
// both parts.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected 1 command-line argument, got {}", args.len() - 1);
        panic!("Usage: \"cargo run <day number>\"");
    }

    let day_num: usize = args[1].parse().expect(
        "Expected number as command-line argument.",
    );

    if day_num <= 0 || day_num > 25 {
        panic!(
            "Invalid day number {}. Only numbers between 1 and 25 are accepted.",
            day_num
        );
    }

    // Array of function pointers, all of which take no parameter and return nothing.
    let days: Vec<fn() -> ()> = vec![
        day_one_wrapper,
        day_two_wrapper,
        day_three_wrapper,
        day_four_wrapper,
        day_five_wrapper,
        day_six,
        day_seven,
        day_eight,
        day_nine,
        day_ten_wrapper,
        day_eleven,
        day_twelve,
        day_thirteen,
        day_fourteen,
        day_fifteen,
        day_sixteen,
        day_seventeen,
        day_eighteen,
        day_nineteen,
        day_twenty,
        day_twenty_one,
        day_twenty_two_wrapper,
        day_twenty_three,
        day_twenty_four,
        day_twenty_five,
    ];
    days[day_num - 1]();
}
