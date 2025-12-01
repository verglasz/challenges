#!/bin/sh
alias dbase='tr -d \" | base64 -di'
alias justpass="sed -e 's/.*://'"
alias nospace="tr -d ' '"
alias pp="xpaste | nospace"
alias getp="justpass | xcopy"

function hc () {
	local file
	file=$1
	shift 1
	hashcat -w3 -m0 "${file}/hash" $@
}

function gps () {
	local file
	file=$1
	shift 1
	cat "${file}/pass" | justpass | justdo $@  | tee /dev/stderr | xcopy
}


function justdo () {
	if [ $# -eq 0 ]; then
		cat
	else
		$@
	fi
}

function allpass () {
	for f in {1..9}-* ; do
		echo ${f}/pass >&2
		cat ${f}/pass | justpass | tr -d '\n' | xargs printf "%s\n"
	done
}

