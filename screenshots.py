#!/usr/bin/env python

from itertools import product
import os
import subprocess

here = os.path.dirname(os.path.realpath(__file__))
executable = here + "/target/release/langtonsant"
destination = here + "/screenshots/"
for length in range(2,7):
    sequnces = ["".join(i) for i in product(['L','R','U','N'], repeat=length)]
    for sequence in sequnces:
        command = f"{executable} -f -m 20000 -r {sequence}"
        subprocess.call(command.split())
        command = f"screencapture -x {destination}/{sequence}.png"
        subprocess.call(command.split())
