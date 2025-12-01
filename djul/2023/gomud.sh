#!/usr/bin/env zsh

set -e
# args: ip:port [commands_file [new_commands]]
ip_port=${1}
ip=${ip_port%:*}
port=${ip_port#*:}
commands_file=${2:-mud.sol}
new_commands=${3:-"$(mktemp "$(realpath ${commands_file}).XXX")"}


nocomment () {
	while read -r line; do
		# remove comments
		line=${line%%#*}
		# remove leading and trailing whitespace
		line=${line## }
		line=${line%% }
		# skip empty lines
		[[ -n "${line}" ]] && printf "%s\n" "${line}"
	done

}

# read lines from file and emit them slowly
slowlines () {
	local sleep=${1:-0.15}
	while read -r line; do
		printf "%s\n" "${line}"
		sleep $sleep
	done
}

# define some command shortcuts
mudshort () {
	local cmd
	# shortcuts: nsew and arrow keys (↑↓→←)
	while read -r cmd; do
		case "${cmd}" in
	 		"n" | $'\e[A' ) printf "go %s\n" "north" ;;
			"s" | $'\e[B' ) printf "go %s\n" "south" ;;
			"e" | $'\e[C' ) printf "go %s\n" "east" ;;
			"w" | $'\e[D' ) printf "go %s\n" "west" ;;
			* ) printf "%s\n" "${cmd}" ;;
		esac
	done
}


# ensure we only append so that at least it's nondestructive
printf "\n# %s\n" "start session for ${ip_port} at $(date --iso-8601=s)" >> "${new_commands}"

# first we take the commands from the file (slowly), printing them on stderr for the user to see
# (if all goes well, they interleave with the output from the mud connection
# in just the right way to make it look like the user is typing the commands)
# then once the file is exhausted we read commands from stdin
# (to let the user continue interactively);
# this commands stream is passed to mudshort, which translates shortcuts,
# then appended to the new_commands file (to keep it for future/iterative use)
# and sent to the mud connection.
#
# some complications:
# we use sponge to read the commands file into memory before we start the mud connection,
# so that we can also use the inpu file as output without breaking everything

# ensure this exists so we don't need to create it when we run it the first time
touch "${commands_file}"

# note tee -a again
cat <(sponge < "${commands_file}" | slowlines | tee /dev/stderr  | nocomment ) - | mudshort |  tee -a "${new_commands}" | nc "${ip}" "${port}"

