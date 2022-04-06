#!/bin/bash

# Copyright (c) 2022 ParallelChain Lab
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

while getopts :f: flag
do
    case "${flag}" in
	f) filename=${OPTARG};;
    esac
done

if test ! -f "$filename"; then
    echo "The file $filename doesn't exist."
    exit 1
fi

w=$(basename -- $filename)

wasm-opt -Oz $filename --output temp-$w
wasm-snip temp-$w --output temp2-$w --snip-rust-fmt-code --snip-rust-panicking-code
wasm-opt --dce temp2-$w --output optimized-$w
rm temp-$w
rm temp2-$w

echo $w `stat -c "%s" $filename` "bytes ->" `stat -c "%s" optimized-$w` "bytes, see minified-$w"
