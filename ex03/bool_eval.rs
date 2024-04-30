fn is_valid(c: char) -> bool {
	return c == '0' || c == '1' || c == '!' || c == '&'
		|| c == '|' || c == '^' || c == '>' || c == '='
}

fn get_result(op: char, a: bool, b: bool) -> bool {
	if op == '&' {
		return a & b;
	} else if op == '|' {
		return a | b;
	} else if op == '^' {
		return a ^ b;
	} else if op == '>' {
		return !a || b
	} else {
		return a == b;
	}
}

pub fn eval_formula(formula: &str) -> bool {
	let mut stack: Vec<bool> = Vec::new();
	for c in formula.chars() {
		if !(is_valid(c)) {
			println!("Invalid character: {}", c);
			return false;
		}

		if c == '0' {
			stack.push(false);
		} else if c == '1' {
			stack.push(true);
		} else if c == '!' {
			if stack.len() < 1 {
				println!("Invalid formula");
				return false;
			}
			let a = stack.pop().unwrap();
			stack.push(!a);
		} else {
			if stack.len() < 2 {
				println!("Invalid formula");
				return false;
			}
			let result = get_result(c, stack.pop().unwrap(), stack.pop().unwrap());
			stack.push(result);
		}
	}
	return stack.pop().unwrap();
}