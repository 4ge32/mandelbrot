#!/bin/sh

cargo run
a=mandel.png

echo "set term png;
set output \"$a\";
set grid;
set pm3d map;
set palette functions gray, gray, gray;
set size square;
unset xtics;
unset ytics
splot \"$a.data\"" | gnuplot

eog $a