use std::thread;
use std::time::Duration;
use colored::Colorize;
use rand::Rng;

fn main() { tannenbaum(23) }

fn tannenbaum(x: u32) {
	let mut top = 0;
	let mut i = 1;
	
	while top < x/2 {
		for _empty_space in 0..2     { print!("{}", " ".on_blue())   }
		top += 1
	}
	if x%2 == 1                      { print!("{}", " ".on_blue())   }
	print!("{}", " ".on_green());
	while top < x - 1 {
		for _empty_right in 0..2     { print!("{}", " ".on_blue())   }
		top += 1
	}
	if x%2 == 1                      { println!("{}", " ".on_blue()) }

	while i <= x {
		let mut line = "".to_string();
		for _empty_left  in 0..(x-i) { line.push('.') }
		for _left_side   in 0..i     { line.push('L') }
		for _middle      in i..i+1   { line.push('M') }
		for _right_side  in 0..i     { line.push('R') }
		for _enpty_right in i..x     { line.push('.') }
		random_lights(x, i, line);
		thread::sleep(Duration::from_millis(120));
		i += 1
	}
	
	for _stem in 0..(x-1)            { print!(" ") }
	println!("{}", "   ".on_green());
}

fn random_lights(range: u32, index: u32, line: String) {
	let mut rng = rand::thread_rng();
	let index_1: usize = rng.gen_range(0..(range * 2) as usize);
	let index_2: usize = rng.gen_range(0..(range * 2) as usize);
	let mut string = line;
	
	string.replace_range(index_1..index_1+1, "x");
	string.replace_range(index_2..index_2+1, "u");
	
	for (i, c) in string.chars().enumerate() {
		let left_side = (range - index) as usize;
		let middle = left_side + (index * 2) as usize;
		if c == 'x' {
			if i < left_side {
				print!("{}", "Ж".on_blue().bright_white())
			} else if i < middle {
				print!("{}", "Ж".on_green().white())
			} else {
				print!("{}", "Ж".on_blue().bright_white())
			}
		} else if c == 'u' {
			print!("{}", " ".on_red())
		} else if c == '.' {
			print!("{}", " ".on_blue())
		} else {
			print!("{}", " ".on_green())
		}
	}
	println!()
}