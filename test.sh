#!/bin/bash
cargo run -- expect -c 'spawn -noecho scp my-server:~/Downloads/100M.bin 100M.bin; expect eof'
