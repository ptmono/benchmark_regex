# - https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/
LIB_NAME=rustlibs

build:
	if [ -a ./${LIB_NAME} ] ; then rm ./${LIB_NAME}; fi;
	if [ -a ../${LIB_NAME} ] ; then rm ../${LIB_NAME}; fi;
	RUSTFLAGS="-C target-cpu=native" cargo build

	@cp -rf ./target/debug/lib${LIB_NAME}.so ./${LIB_NAME}.so
	@cp -rf ./target/debug/lib${LIB_NAME}.so ../${LIB_NAME}.so
	python3.6 test.py


test:
	cargo test -- --nocapture
