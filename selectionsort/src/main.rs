extern crate ansi_escapes;
extern crate rand;
extern crate time;

use std::io::{stdin,stdout,Write};
use std::{thread};
use rand::{Rng};
use time::PreciseTime;

static IS_BENCH: bool = false;
macro_rules! onbench {
  ($x:expr) => ( if IS_BENCH { $x; } );
}

fn print_data(data: &Vec<i32>) {
  onbench!(return);
  println!("{:?}", data);
}

fn pretty_print_data(data: &Vec<i32>, p: usize, min: usize, max: usize) {
  onbench!(return);
  print!("{}", ansi_escapes::CursorRestorePosition);
  for (i, e) in data.iter().enumerate() {
    print!("{}", ansi_escapes::EraseLine);
    for _i in 0..*e {
      print!("█");
    }
    if i == min { print!(" <<<<"); }
    if i == max { print!(" >>>>") }
    if i == p { print!(" <###"); } 
    println!();
  }
  stdout().flush().ok();
}

fn sleep() {
  if !IS_BENCH {
    thread::sleep(std::time::Duration::from_millis(50));
  }
}


fn swap(data: &mut Vec<i32>, i: &usize, j: &usize) {
  if *i == *j { return; }
  let t = data[*j];
  data[*j] = data[*i];
  data[*i] = t;
}


fn selection_sort(data: &mut Vec<i32>) {
  let len: usize = data.len() as usize;
  let mut min: usize;
  for i in 0..(len) {
    min = i;
    for cursor in i..len {
      if data[cursor] < data[min] { min = cursor; }
      pretty_print_data(data, cursor, min, len);
      sleep();
    }
    swap(data, &i, &min);
  }
}

// keeps track of both minimum and maximum
fn cocktail_sort(data: &mut Vec<i32>) {
  let len: usize = data.len() as usize;
  let mut min: usize; let mut max: usize;
  let mut i: usize = 0; let mut j: usize = len-1;
  while i < j {
    min = i; max = j;
    for cursor in i..(j+1) {
      if      data[cursor] < data[min] { min = cursor; }
      else if data[cursor] > data[max] { max = cursor; }
      pretty_print_data(data, cursor, min, max);
      sleep();
    }

    if max == i && min == j {
      swap(data, &i, &j);
    } else {
      swap(data, &i, &min);
      if i == max { max = min; } // max has changed position
      swap(data, &j, &max);
    }
    i += 1; j -= 1;
  }
  pretty_print_data(data, i, len, len);
}


fn main() {
  let mut input: String = String::new();
  stdin().read_line(&mut input).ok();
  let cap: usize = input.trim().parse::<usize>().unwrap();

  let mut data: Vec<i32> = Vec::<i32>::with_capacity(cap);
  for i in 0..cap {
    data.push((i as i32)+1);
    // data.push((cap-i) as i32);
    // data.push(10);
  }

  let mut rng = rand::thread_rng();
  rng.shuffle(&mut data);

  println!("finished generating data...");
  print_data(&data);
  print!("{}", ansi_escapes::CursorSavePosition);
  let start = PreciseTime::now();
  cocktail_sort(&mut data);
  let end = PreciseTime::now();
  print_data(&data);
  println!("{} elements took {} to sort", cap, start.to(end));
}
