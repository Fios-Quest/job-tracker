$PHONY=%

dev:
	npx @tailwindcss/cli -i ./input.css -o ./desktop/assets/generated/tailwind.css --watch & \
		dx serve --package job-tracker

pre-commit:
	cargo fmt --check & \
	    cargo check & \
	    cargo build & \
	    cargo clippy & \
	    cargo test & \
	    dx fmt --check
