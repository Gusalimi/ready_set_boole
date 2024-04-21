pub fn adder(a: u32, b: u32) -> u32 {
	if b == 0 {
		return a;
	}

	return adder(a ^ b, (a & b) << 1);
}
