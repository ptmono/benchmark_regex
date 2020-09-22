init-env:
	python3.6 -m venv _venv
	. _venv/bin/activate; pip install -r requirements.txt

rust-test:
	@cd rust_regex; make test

rust-build:
	@cd rust_regex; . ../_venv/bin/activate; make build

benchmark: rust-build
	@cp ./rust_regex/rustlibs.so ./
	. ./_venv/bin/activate; python3.6 benchmark.py
