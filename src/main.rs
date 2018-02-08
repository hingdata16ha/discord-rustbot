mod config;
mod bot;
use bot::make_bot;

fn main() {
	let config = config::get_config();
	make_bot(config.token);
}
