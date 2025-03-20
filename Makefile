all: build

build:
	cargo build --release

install: build
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -d ./templates/ /etc/walrs/templates
	sudo cp -r templates/* /etc/walrs/templates/

uninstall:
	sudo rm -f /usr/bin/walrs
	sudo rm -rf /etc/walrs/

clean:
	cargo clean

.PHONY: all build install uninstall clean
