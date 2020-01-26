#!/bin/sh

# Requires compiled LinuxCNC project next to crate

gcc -DULAPI -I../../../linuxcnc/include test.c -L../../../linuxcnc/lib -llinuxcnchal -o test
