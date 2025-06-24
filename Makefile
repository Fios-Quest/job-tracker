$PHONY=%

dev:
	npx @tailwindcss/cli -i ./input.css -o ./desktop/assets/generated/tailwind.css --watch & \
		dx serve --package job-tracker

pre-commit:
	cargo check
	cargo build
	cargo test
	cargo fmt --check
	cargo clippy -- -D warnings
	dx fmt --check
