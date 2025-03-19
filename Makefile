all: build

build:
	cargo build --release

install: build
	sudo install -d $(DESTDIR)$(PREFIX)/bin
	sudo install -m755 target/release/walrs /usr/bin/walrs
	sudo install -d ./templates/ /etc/walrs/templates
	sudo cp -r templates/* /etc/walrs/templates/

uninstall:
	sudo rm -f $(DESTDIR)$(PREFIX)/bin/walrs
	sudo rm -rf $(DESTDIR)$(SYSCONFDIR)/walrs

clean:
	cargo clean

.PHONY: all build install uninstall clean
