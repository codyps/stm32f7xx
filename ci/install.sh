#! /bin/bash
set -ex

: ${TARGET:=thumbv7em-none-eabihf}
main() {
    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        rustup target add $TARGET
    fi
}

main
