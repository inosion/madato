wasm_compiled = www-sample/markdown_table_maker_bg.wasm www-sample/markdown_table_maker.d.ts www-sample/markdown_table_maker.js
wasm_debug    = target/wasm32-unknown-unknown/debug/markdown_table_maker.wasm

$(wasm_debug):
	cargo +nightly build --target wasm32-unknown-unknown

$(wasm_compiled): $(wasm_debug)
	wasm-bindgen $(wasm_debug) --out-dir www-sample

all: $(wasm_compiled)

clean:
	rm -f $(wasm_compiled) $(wasm_debug)

.PHONY: all
