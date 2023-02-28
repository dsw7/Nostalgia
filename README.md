# Nostalgia
A small program for accurately measuring the capacitance of through-hole capacitors. Essentially, I just built
an astable multivibrator (using an NE555P timer) and I connect any capacitor to the circuit. I then measure
the period of the emitted square wave using an oscilloscope. This software then converts the extracted period
to a capacitance value.
## Table of Contents
- [Software](#software)
    - [Theory](#theory)
- [Hardware](#hardware)

## Software
### Theory
The Texas Instruments datasheet for the NE555P timer states that the frequency $f$ of the output waveform
follows:

$$
f = \frac{1.44}{C(R_1 + 2R_2)}
$$

This equation can be rearranged to extract solve for capacitance, $C$:

$$
C = \frac{1.44}{f(R_1 + 2R_2)}
$$

And $f$ is the inverse of the period, $p$:

$$
f = 1 / p
$$

## Hardware
