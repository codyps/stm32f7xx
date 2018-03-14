#! /bin/sh
d="$(dirname "$0")"
for i in "$d/svd"/*; do
	case "$i" in
	*.svd)
		b="$(basename "$i" .svd)"
		l="$(printf "%s" "$b" | tr '[:upper:]' '[:lower:]')"
		(
			cd "$d/$l"
			"$@"
		)
		;;
	*)
		2>&1 echo "skipping"
		;;
	esac
done

