#!/bin/bash
cargo run -- ssh -t -q -A localhost scp my-server:~/Downloads/100M.bin 100M.bin