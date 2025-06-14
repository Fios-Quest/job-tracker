$PHONY=%

dev:
	npx @tailwindcss/cli -i ./input.css -o ./desktop/assets/generated/tailwind.css --watch & \
		dx serve --package job-tracker
