PREFIX ?= /usr

all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo mkdir -p /etc/walrs/templates/ /etc/walrs/scripts/ /etc/walrs/colorschemes/
	sudo cp -r templates/* /etc/walrs/templates/
	sudo cp -r scripts/* /etc/walrs/scripts/
	sudo cp -r colorschemes/* /etc/walrs/colorschemes/
	sudo install -m644 walrs.1 /usr/share/man/man1/
	bash ./autocomplete.sh


nix:
	nix build
	sudo cp ./result/bin/walrs /usr/bin/walrs
	sudo mkdir -p /etc/walrs
	sudo cp -r ./result/etc/walrs/* /etc/walrs/
	sudo cp ./result/share/man/man1/walrs.1 /usr/share/man/man1/walrs.1clean:
	cargo clean


.PHONY: all build install uninstall clean nix

