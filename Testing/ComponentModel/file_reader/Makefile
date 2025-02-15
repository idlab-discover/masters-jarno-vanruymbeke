wac:
	wac plug invoker/target/wasm32-wasip1/release/invoker.wasm --plug file_reader/target/wasm32-wasip1/release/file_reader.wasm -o final.wasm

compile_file_reader:
	cd file_reader && cargo component build --release

compile_invoker:
	cd invoker && cargo component build --release

compile: compile_file_reader compile_invoker
