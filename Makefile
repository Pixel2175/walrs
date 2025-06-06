PREFIX ?= /usr

all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -m644 walrs.1 /usr/share/man/man1/
	mkdir -p ~/.cache/walrs/templates/ ~/.cache/walrs/scripts/ ~/.cache/walrs/colorschemes/
	cp -r templates/* ~/.cache/walrs/templates/
	cp -r scripts/* ~/.cache/walrs/scripts/
	cp -r colorschemes/* ~/.cache/walrs/colorschemes/
	bash ./autocomplete.sh
uninstall:
	sudo rm -rf /usr/bin/walrs ~/.cache/walrs ~/.config/walrs/ /usr/share/man/man1/walrs.1


nix:
	nix build
	mkdir -p ~/.cache/walrs/templates/ ~/.cache/walrs/scripts/ ~/.cache/walrs/colorschemes/
	sudo cp ./result/bin/walrs ~/.local/bin/ 
	cp -r templates/* ~/.cache/walrs/templates/
	cp -r scripts/* ~/.cache/walrs/scripts/
	cp -r colorschemes/* ~/.cache/walrs/colorschemes/
	bash ./autocomplete.sh
clean:
	cargo clean


.PHONY: all build install uninstall clean nix

