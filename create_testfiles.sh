#! /bin/bash


mkdir -p tmp/testfiles 
cd tmp/testfiles
for i in {1..2000} 
    do
        FILESIZE=$(( $RANDOM % 100000 + 1 ))
        head -c $FILESIZE </dev/urandom > tmp/testfiles/file$i
    done