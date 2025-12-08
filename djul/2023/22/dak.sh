#!/usr/bin/env zsh


readprog () {
	prog=""
	while read -r line; do
		prog="$prog $line ."
	done
	printf "%s\n" "$prog"
}

while true; do
	readprog
done

