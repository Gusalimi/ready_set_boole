mod multiplier;

fn main() {
	println!("Product of {} and {} is {}", 3, 5, multiplier::multiplier(3, 5));
	println!("Product of {} and {} is {}", 0, 0, multiplier::multiplier(0, 0));
	println!("Product of {} and {} is {}", 0, 1, multiplier::multiplier(0, 1));
	println!("Product of {} and {} is {}", 1, 0, multiplier::multiplier(1, 0));
	println!("Product of {} and {} is {}", 1, 1, multiplier::multiplier(1, 1));
	println!("Product of {} and {} is {}", 1, 2, multiplier::multiplier(1, 2));
	println!("Product of {} and {} is {}", 2, 1, multiplier::multiplier(2, 1));
	println!("Product of {} and {} is {}", 2, 2, multiplier::multiplier(2, 2));
}