TEST_DB_URL=$(shell cat .env.test | grep TEST_DATABASE_URL | cut -d '=' -f2 | tr -d '"')


deploy-local:
	docker compose up -d
	diesel migration run

migrations:
	diesel migration run

migration-reverts:
	diesel migration revert

migrations-test:
	diesel migration run --database-url $(TEST_DB_URL)

migration-revert-test:
	diesel migration revert --database-url $(TEST_DB_URL)

sync:
	cargo run sync