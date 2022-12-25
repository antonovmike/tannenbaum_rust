use std::thread;
use std::time::Duration;
use colored::Colorize;

fn main() { tannenbaum(23); }

fn tannenbaum(x: u32) {
	let mut top = 0;
	let mut i = 1;
	// let mut line = "".to_string();
	
	while top < x/2 {
		for _empty_space in 0..2     { print!(" "); }
		top += 1
	}
	if x%2 == 1 { print!(" ") }
	println!("{}", " ".on_yellow());

	while i <= x {
		let mut line = "".to_string();
		for _empty_space in 0..(x-i) { 
			// print!(" ");
			line.push(' ') 
		}
		for _left_side   in 0..i     { 
			// print!("{}", " ".on_green()); 
			line.push('L')
		}
		for _middle      in i..i+1   { 
			// print!("{}", " ".on_blue()); 
			line.push('M')
		}
		for _right_side  in 0..i     { 
			// print!("{}", " ".on_magenta()); 
			line.push('R')
		}
		println!("{line}");
		thread::sleep(Duration::from_millis(120));
		i += 1
	}
	
	for _stem in 0..(x-1) { print!(" "); }
	println!("{}", "   ".on_blue());
}
