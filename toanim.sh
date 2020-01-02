#!/bin/bash

rm output/*
cargo run
convert -delay 20 -loop 0 output/*.pbm output/anim.gif
convert -delay 5 -quality 95 output/*.pbm output/anim.mpg