#!/usr/bin/env zsh

for img in vacation_pics/*; do
    word=$(./png-text-embed/png-text-dump "$img" ) #| sed -n '2p')
    echo $img
    echo $word
done

