use std::thread;
use std::time::Duration;

fn main() { tannenbaum(23) }

fn tannenbaum(x: i32) {
	let mut top = 0;
	let mut i = 1;

	while top < x/2 {
		for _empty_space in 0..2     { print!(" ") }
		top += 1
	}

	if x%2 == 1                      { print!(" ") }
	
	println!("A");

	while i <= x {
		for _empty_space in 0..(x-i) { print!(" ") }
		for _left_side   in 0..i     { print!("L") }
		for _middle      in i..i+1   { print!("M") }
		for _right_side  in 0..i     { print!("R") }
		println!();
		thread::sleep(Duration::from_millis(120));
		i += 1
	}
	
	for _stem in 0..(x-1) 			 { print!(" ") }

	println!("ШШШ");
}
