#!/bin/sh

a=mtest.png
b=-2
c=0.6
d=-1.2
e=1.2
f=0.0015
g=20

b=-2
c=2
d=-2
e=2
f=0.010
g=20

rainbow="22, 13, -31"
mono="grey"

python3 mandel.py $a.data $b $c $d $e $f $g

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
