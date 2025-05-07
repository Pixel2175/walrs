all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -d ./templates/ /etc/walrs/templates
	/usr/bin/walrs --install-completions -q
	sudo cp -r templates/ /etc/walrs/
	sudo cp -r colorschemes/ /etc/walrs/

uninstall:
	sudo rm -f /usr/bin/walrs
	sudo rm -rf /etc/walrs/
	sudo rm -rf ~/.config/walrs

clean:
	cargo clean

.PHONY: all build install uninstall clean
