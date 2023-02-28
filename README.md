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

This equation can be rearranged to solve for the capacitance, $C$:

$$
C = \frac{1.44}{f(R_1 + 2R_2)}
$$

And $f$ is the inverse of the period, $p$:

$$
f = 1 / p
$$

$R_1$ and $R_2$ were measured at 99.8 and 21.6 $k\Omega$, respectively. Therefore, the final equation becomes

$$
C = \frac{1.44}{143000\Omega / p}
$$

Where $p$ is obtained directly from the oscilloscope and is measured in seconds.
## Hardware
<p align="center">
  <img width="200" src=docs/example_555_timer.png>
</p>
