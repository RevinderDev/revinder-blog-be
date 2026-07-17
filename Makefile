all:
	cargo watch --ignore 'hurl/' -x  'clippy -- -D warnings -W clippy::all' -x 'test' -x 'run'
