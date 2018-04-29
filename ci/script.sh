#! /bin/bash
set -eux -o pipefail
d="$(dirname "$0")/.."
: ${TARGET:=thumbv7em-none-eabihf}
main() {
	for i in "$d/stm32f7"*; do
		(
			cd "$i"
			xargo check --target $TARGET
		)
	done
}

main
