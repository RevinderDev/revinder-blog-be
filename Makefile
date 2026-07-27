all:
	cargo watch --ignore 'hurl/' -x  'clippy -- -D warnings -W clippy::all' -x 'run'

format:
	cargo fix --allow-dirty

db-up:
	sqlx migrate run

db-down: 
	sqlx migrate revert

db-shell:
	sqlite3 revinder_blog_be.db
