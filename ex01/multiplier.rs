pub fn adder(a: u32, b: u32) -> u32 {
	if b == 0 {
		return a;
	}

	return adder(a ^ b, (a & b) << 1);
}

pub fn multiplier(a: u32, b: u32) -> u32 {
	let mut result: u32 = 0;
	let mut i: u32 = 0;

	while i < b {
		result = adder(result, a);
		i += 1;
	}
	return result;
}