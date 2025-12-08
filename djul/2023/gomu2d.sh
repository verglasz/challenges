#!/usr/bin/env zsh

set -e
# args: ip:port [commands_file [new_commands]]

typeset do_debug=0
typeset -a args=( [2]="mud.sol" )

parseargs () {
	# parse arguments
	local i=1
	while [[ $# -gt 0 ]]; do
		case "${1}" in
			--help | -h )
				printf "%s\n" "usage: ${0} ip:port [commands_file [new_commands]]"
				exit 0
				;;
			--debug )
				do_debug=1
				shift
				;;
			* )
				args[$i]="${1}"
				(( i++ ))
				shift
				;;
		esac
	done
	if [[ -z "${args[1]}" ]]; then
		printf "%s\n" "usage: ${0} ip:port [commands_file [new_commands]]"
		exit 1
	fi
	if [[ "${#args}" -lt 3 ]]; then
		base=$(realpath ${args[2]})
		args[3]="$(mktemp "$base.XXX")"
	fi

}

parseargs "${@}"

ip_port=${args[1]}
commands_file=${args[2]}
new_commands=${args[3]}

ip=${ip_port%:*}
port=${ip_port#*:}

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

if [[ "${do_debug}" -eq 1 ]]; then
	debug () {
		tee "${1:-/dev/stderr}"
	}
else
	debug () {
		cat
	}
fi

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
			"da "* ) printf "dankan %s\n" "${cmd#*' '}" ;;
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

# up to "nocomment" we're reading the old commands,
# cat continues when they end by taking interactive user input,
# then we process the commamnd stream and send it to the mud connection (and save it)
# note tee -a again
sponge < "${commands_file}" |
	slowlines |
	tee /dev/stderr  |
	nocomment  |
	cat - <(cat) |
	mudshort |
	tee -a "${new_commands}" |
	debug |
	nc "${ip}" "${port}"
