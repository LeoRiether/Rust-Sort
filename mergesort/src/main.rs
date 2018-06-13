extern crate ansi_escapes;

static IS_DEBUG: bool = false;
macro_rules! debug {
  ($x:expr) => ( if IS_DEBUG { $x; } );
}
macro_rules! no_debug {
  ($x:expr) => ( if !IS_DEBUG { $x; } );
}

use std::io::{stdin,stdout,Write};
use std::{thread,time};

fn print_data(data: &Vec<i32>) {
  debug!(return);
  println!("{:?}", data);
  // print!("[ ");
  // for e in data.iter() {
  //   print!("{} ", e);
  // }
  // println!("]");
}

fn pretty_print_data(data: &Vec<i32>) {
  debug!(return);
  print!("{}", ansi_escapes::CursorRestorePosition);
  for e in data.iter() {
    print!("{}", ansi_escapes::EraseLine);
    for _i in 0..*e {
      print!("█");
    }
    println!();
  }
  stdout().flush().ok();
}

fn merge(data: &mut Vec<i32>, aux: &mut Vec<i32>, size: usize) {
  let mut lower_bound: usize = 0;
  let mut left_cursor: usize = 0;
  let mut right_cursor: usize = size;
  let mut i: usize = 0;
  while lower_bound < data.len() {
    loop {

      // Populate with the rest of the right side
      if left_cursor == lower_bound + size || left_cursor >= data.len() {
        while right_cursor < lower_bound + 2*size && right_cursor < data.len() {
          aux[i] = data[right_cursor];
          i += 1; right_cursor += 1;
        }
        break;
      }
      // Populate with the rest of the left side 
      else if right_cursor == lower_bound + 2*size || right_cursor >= data.len() {
        while left_cursor < lower_bound + size && left_cursor < data.len() {
          aux[i] = data[left_cursor];
          i += 1; left_cursor += 1;
        }
        break;
      }

      if data[left_cursor] <= data[right_cursor] {
        aux[i] = data[left_cursor];
        left_cursor += 1;
      } else {
        aux[i] = data[right_cursor];
        right_cursor += 1;
      }
      i += 1;

    }

    lower_bound = lower_bound + 2*size;
    left_cursor = lower_bound;
    right_cursor = lower_bound + size;
  }

  std::mem::swap(data, aux);
}

fn main() {
  // initialize variables
  let mut data: Vec<i32> = Vec::new();
  let mut aux : Vec<i32> = Vec::new();
  let sleep_time = time::Duration::from_millis(500);

  // Read vector
  let mut input: String;
  loop {
    input = String::new();
    stdin().read_line(&mut input).ok();
    if input.trim() == "e" { break; }
    data.push(input.trim().parse::<i32>().unwrap());
  }

  // Allocate aux vector
  aux.reserve(data.capacity());
  for _i in data.iter() {
    aux.push(0);
  }

  print_data(&data);

  // Save position
  print!("{}", ansi_escapes::CursorSavePosition);

  pretty_print_data(&data);
 
  // Do the actual merge sorting
  let mut size: usize = 1;
  while size < data.len() {
    no_debug!(thread::sleep(sleep_time));
    merge(&mut data, &mut aux, size);
    pretty_print_data(&data);
    size *= 2;
  }

  print_data(&data);
  
}