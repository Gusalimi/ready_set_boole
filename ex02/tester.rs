mod gray;

fn main() {
	let mut n: u32 = 0;
	while n < 16 {
		println!("Gray code of {} is {}", n, gray::gray_code(n));
		n += 1;
	}
}