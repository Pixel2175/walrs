all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -d ./templates/ /etc/walrs/templates
	sudo cp -r templates/ /etc/walrs/
	bash ./autocomplete.sh
	sudo cp ./walrs.1 /usr/share/man/man1/
	sudo cp -r colorschemes/ /etc/walrs/

uninstall:
	sudo rm -rf /usr/bin/walrs /etc/walrs/ ~/.config/walrs/ /usr/share/man/man1/walrs.1
clean:
	cargo clean

.PHONY: all build install uninstall clean
