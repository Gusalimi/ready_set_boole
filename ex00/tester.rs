mod adder;

fn main() {
	let mut a: u32 = 5;
	let mut b: u32 = 3;
	println!("Sum of {} and {} is {}", a, b, adder::adder(a, b));
	a = 0;
	b = 0;
	println!("Sum of {} and {} is {}", a, b, adder::adder(a, b));
	a = 0;
	b = 1;
	println!("Sum of {} and {} is {}", a, b, adder::adder(a, b));
	a = 1;
	b = 0;
	println!("Sum of {} and {} is {}", a, b, adder::adder(a, b));
}