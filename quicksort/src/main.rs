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

fn pretty_print_data(data: &Vec<i32>, p: usize) {
  onbench!(return);
  print!("{}", ansi_escapes::CursorRestorePosition);
  for (i, e) in data.iter().enumerate() {
    print!("{}", ansi_escapes::EraseLine);
    for _i in 0..*e {
      print!("█");
    }
    if i == p { print!(" <<<<"); } 
    println!();
  }
  stdout().flush().ok();
}

fn sleep() {
  if !IS_BENCH {
    thread::sleep(std::time::Duration::from_millis(75));
  }
}

// PARTITIONING

fn swap(data: &mut Vec<i32>, i: usize, j: usize) {
  let t = data[j];
  data[j] = data[i];
  data[i] = t;
}

fn hoare_partition(data: &mut Vec<i32>, a: usize, b: usize) -> usize {
  let p = data[a];
  let mut i = a;
  let mut j = b+1;
  loop {
    j -= 1;
    while data[j] > p { j -= 1; }

    i += 1;
    while data[i-1] < p { i += 1; }

    pretty_print_data(data, j);
    sleep();
    if (i-1) < j {
      swap(data, i-1, j);
    } else {
      return j;
    }
  }
}

fn loumoto_partition() {

}

// RECURSIVE QUICK SORT

fn quick_sort(data: &mut Vec<i32>, a: usize, b: usize) {
  if a >= b { return; }
  let p: usize = hoare_partition(data, a, b);
  quick_sort(data, a, p);
  quick_sort(data, p+1, b);
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
  quick_sort(&mut data, 0, cap-1);
  // data.sort();
  let end = PreciseTime::now();
  print_data(&data);
  println!("{} elements took {} to sort", cap, start.to(end));
}
