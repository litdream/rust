#!/bin/bash

src=$1

[[ -z $src ]] && {
	echo "need source file to compile"
	exit 2
}

rustc -o a.out $src
