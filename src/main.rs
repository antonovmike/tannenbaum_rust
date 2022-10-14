use std::thread;
use std::time::Duration;
use colored::Colorize;

fn main() { tannenbaum(7); }

fn tannenbaum(x: i32) {
	let mut top = 0;
	let mut i = 1;

	while top < x/2 {
		for _empty_space in 0..2     { print!(" "); }
		top += 1
	}
	if x%2 == 1 { print!(" ") }
	println!("{}", "A".on_yellow());

	while i <= x {
		for _empty_space in 0..(x-i) { print!(" "); }
		for _left_side   in 0..i     { print!("{}", "L".on_green()); }
		for _middle      in i..i+1   { print!("{}", "M".on_blue()); }
		for _right_side  in 0..i     { print!("{}", "R".on_magenta()); }
		println!();
		thread::sleep(Duration::from_millis(120));
		i += 1
	}
	
	for _stem in 0..(x-1) { print!(" "); }
	println!("{}", "ШШШ".on_red());
}
