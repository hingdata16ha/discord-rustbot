pub struct Bot<'a> {
	token: &'a str,
}

pub fn make_bot(token: String) {
	println!("{}", token);
}
