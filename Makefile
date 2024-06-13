.PHONY: run migrate prepare

all:run

migrate:
	sqlx migrate run

prepare:
	cargo sqlx prepare

run:
	cargo run
