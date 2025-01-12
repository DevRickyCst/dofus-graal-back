migrations:
	diesel migration run

migration-reverts:
	diesel migration revert

sync:
	cargo run sync