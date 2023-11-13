#!/bin/bash
cargo run -- winpty -Xallow-non-tty -Xplain scp my-server:~/Downloads/100M.bin 100M.bin