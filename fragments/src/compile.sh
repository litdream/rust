#!/bin/bash

# compile with bin-name
#   - cargo build --bin bull-cow
#   - cargo run  --bin bull-cow
#

src=$1

[[ -z $src ]] && {
	echo "need source file to compile"
	exit 2
}

#rustc -o a.out $src
cd ../
cargo build --bin $src

