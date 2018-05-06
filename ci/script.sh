#! /bin/bash
set -ex
d="$(dirname "$0")/.."
: ${TARGET:=thumbv7em-none-eabihf}
export RUSTFLAGS="-C codegen-units=1"
main() {
	for i in "$d/stm32f7"*; do
		( 
		cd "$i"
		cargo check --target $TARGET --verbose
		)
	done
}

main
