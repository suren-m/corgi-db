install-all: install-server \
	install-client \
	install-control-plane \
	install-local-cluster-bootstrap

uninstall-all: uninstall-server \
	uninstall-client \
	uninstall-control-plane \
	uninstall-local-cluster-bootstrap


install-server:
	cargo install --path server

install-client:
	cargo install --path client

install-control-plane:
	cargo install --path control-plane

install-local-cluster-bootstrap:
	cargo install --path local-cluster-bootstrap


uninstall-server:
	cargo uninstall -p server

uninstall-client:
	cargo uninstall -p client

uninstall-control-plane:
	cargo uninstall -p control-plane

uninstall-local-cluster-bootstrap:
	cargo uninstall -p local-cluster-bootstrap

init-local-data:
	cp ./words ~/.corgi/data/