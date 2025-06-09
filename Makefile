$PHONY=%

dev:
	npx @tailwindcss/cli -i ./input.css -o ./desktop/assets/tailwind.css --watch & \
		dx serve --package job-tracker
