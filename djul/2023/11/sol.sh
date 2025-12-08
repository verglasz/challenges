#!/bin/sh

set -e
ARCHIVE="https://web.archive.org"
curl -L "$ARCHIVE/web/20230523111458/https://djulkalendern.se/" | rg 'img src' | sed -r 's/^.*img src="([^"]*)".*$/\1/' > images.list
{
    read -r line
    printf "requesting %s\n" "$ARCHIVE$line"
    curl -L "${ARCHIVE}${line}" -o "img1.png"
    read -r line
    curl -L "${ARCHIVE}${line}" -o "img2.png"
} < images.list

compare img1.png img2.png -compose src diff.png
tesseract diff.png stdout

