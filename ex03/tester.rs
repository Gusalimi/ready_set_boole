mod bool_eval;

fn main() {
	println!("{}", bool_eval::eval_formula("10&")); // false
	println!("{}", bool_eval::eval_formula("10|")); // true
	println!("{}", bool_eval::eval_formula("11>")); // true
	println!("{}", bool_eval::eval_formula("10=")); // false
	println!("{}", bool_eval::eval_formula("1011||=")); // true
}

// TODO:
// Demander a flo:
// - Comment gérer le ! (!= ou pas prochaine demande)