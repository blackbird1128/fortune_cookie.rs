
install:
	cargo install --path .
	sudo mkdir -p /usr/local/share/fortune-cookie
	sudo cp -r ./fortunes/* -t /usr/local/share/fortune-cookie
