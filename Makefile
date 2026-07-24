all:
	cargo watch --ignore 'hurl/' -x  'clippy -- -D warnings -W clippy::all' -x 'test' -x 'run'

format:
	cargo fix --allow-dirty

up:
	sqlx migrate run

down: 
	sqlx migrate revert
