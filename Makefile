all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -m644 walrs.1 /usr/share/man/man1/
	sudo mkdir -p /usr/share/walrs/templates/ /usr/share/walrs/scripts/ /usr/share/walrs/colorschemes/
	sudo cp -r templates/* /usr/share/walrs/templates/
	sudo cp -r scripts/* /usr/share/walrs/scripts/
	sudo cp -r colorschemes/* /usr/share/walrs/colorschemes/
	bash ./autocomplete.sh
uninstall:
	sudo rm -rf /usr/bin/walrs /usr/share/walrs ~/.config/walrs/ /usr/share/man/man1/walrs.1


nix:
	nix build
	sudo install -m755 result/bin/walrs /usr/bin/walrs
	sudo mkdir -p /usr/share/walrs/templates/ /usr/share/walrs/scripts/ /usr/share/walrs/colorschemes/
	sudo cp -r templates/* /usr/share/walrs/templates/
	sudo cp -r scripts/* /usr/share/walrs/scripts/
	sudo cp -r colorschemes/* /usr/share/walrs/colorschemes/
	bash ./autocomplete.sh
clean:
	cargo clean


.PHONY: all build install uninstall clean nix

