all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo mkdir -p /etc/walrs/templates/  /etc/walrs/scripts/ /etc/walrs/colorschemes/
	sudo cp -r templates/* /etc/walrs/templates/
	sudo cp -r scripts/* /etc/walrs/scripts/
	sudo cp -r colorschemes/* /etc/walrs/colorschemes/
	sudo cp ./walrs.1 /usr/share/man/man1/
	bash ./autocomplete.sh

uninstall:
	sudo rm -rf /usr/bin/walrs /etc/walrs/ ~/.config/walrs/ /usr/share/man/man1/walrs.1
clean:
	cargo clean

.PHONY: all build install uninstall clean
