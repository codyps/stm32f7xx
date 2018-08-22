#! /bin/bash
set -eu -o pipefail
d="$(dirname "$0")"
for i in "$d/svd"/*; do
	case "$i" in
	*.svd)
		echo "generate $i"
		b="$(basename "$i" .svd)"
		l="$(printf "%s" "$b" | tr '[:upper:]' '[:lower:]')"
		if ! [ -e "$d/$l" ] ; then
			echo " new"
			cargo new --lib "$d/$l"
			sed -i "$d/$l/Cargo.toml" \
				-e '/version/r crate.txt' \
				-e '/\[dependencies\]/r deps.txt' \
				-e 's/version =.*/version = "0.3.0"/'

			# split because `-e` doesn't affect data inserted in the same invocation
			sed -i "$d/$l/Cargo.toml" \
				-e 's/@NAME@/'$b'/g'
		fi
		(
			cp "$i" "$d/$l/svd.svd"
			cd "$d/$l"
			svd2rust -i svd.svd
			rm svd.svd
			rm -rf src
			form -i lib.rs -o src && rm lib.rs
			cargo fmt
		)
		;;
	*)	2>&1 echo "skipping $i"
	esac
done
