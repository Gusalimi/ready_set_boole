pub fn adder(a: u32, b: u32) -> u32 {
	// if no more carry, return the sum
	if b == 0 {
		return a;
	}

	// 1. a ^ b get the sum of the bits without the carry
	// 2. (a & b) << 1 get the carry (in the next bit)
	// 3. recursivity to add the carry to the sum
	return adder(a ^ b, (a & b) << 1);
}
