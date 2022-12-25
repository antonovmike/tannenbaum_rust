use std::thread;
use std::time::Duration;
use colored::Colorize;
// extern crate rand;
use rand::Rng;

fn main() { tannenbaum(23) }

fn tannenbaum(x: u32) {
	let mut top = 0;
	let mut i = 1;
	// let mut line = "".to_string();
	
	while top < x/2 {
		for _empty_space in 0..2     { print!("{}", " ".on_blue()); }
		top += 1
	}
	if x%2 == 1 { print!("{}", " ".on_blue()) }
	print!("{}", " ".on_green());
	while top < x - 1 {
		for _empty_right in 0..2 { print!("{}", " ".on_blue())}
		top += 1
	}
	if x%2 == 1 { println!("{}", " ".on_blue())}
	// println!();

	while i <= x {
		let mut line = "".to_string();
		for _empty_left  in 0..(x-i) {line.push('.')}
		for _left_side   in 0..i     {line.push('L')}
		for _middle      in i..i+1   {line.push('M')}
		for _right_side  in 0..i     {line.push('R')}
		for _enpty_right in i..x     {line.push('.')}
		// println!("{line}");
		random_lights(x, line);
		thread::sleep(Duration::from_millis(120));
		i += 1
	}
	
	for _stem in 0..(x-1) { print!(" "); }
	println!("{}", "   ".on_green());
}

fn random_lights(range: u32, line: String) {
	let mut rng = rand::thread_rng();
	let index_1: usize = rng.gen_range(0..(range * 2) as usize);
	let index_2: usize = rng.gen_range(0..(range * 2) as usize);
	let mut string = line;
	
	string.replace_range(index_1..index_1+1, "x");
	string.replace_range(index_2..index_2+1, "u");
	
	for c in string.chars() {
		if c == 'x' {
			print!("{}", c.to_string().on_bright_yellow())
			// print!("{c}")
		} else if c == 'u' {
			print!("{}", c.to_string().on_red())
		} else if c == '.' {
			print!("{}", c.to_string().on_blue())
		} else {
			print!("{}", c.to_string().on_green())
		}
		// print!("{c}");
	}
	println!()
}