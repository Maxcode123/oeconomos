default:

install-diesel-cli:
	cargo install diesel_cli --no-default-features --features sqlite

create-db:
	diesel setup