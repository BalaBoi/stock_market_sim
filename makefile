.PHONY: dev db-up db-down migrate-up

dev:
	cargo watch -x run

db-up:
	docker-compose up -d postgres

db-down:
	docker-compose down

migrate-up:
	sqlx migrate run
