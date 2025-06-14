#!/bin/sh

set -e
#set -x

here=$(dirname $(realpath $0))
checkinks_tool_path=$(dirname $here)/checklink

( cd $checkinks_tool_path && cargo run $here )
