deploy-local:
	docker compose up -d
	diesel migration run

migrations:
	diesel migration run

migration-reverts:
	diesel migration revert

sync:
	cargo run sync