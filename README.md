# Nostalgia
A small program for accurately measuring the capacitance of through-hole capacitors. Essentially, I just built
an astable multivibrator (using an NE555P timer) and I connect any capacitor to the circuit. I then measure
the period of the emitted rectangular wave using an oscilloscope. This software then converts the extracted
period to a capacitance value.
## Table of Contents
- [Software](#software)
    - [Theory](#theory)
    - [Setup](#setup)
- [Hardware](#hardware)
    - [Usage](#usage)

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

Where $p$ is obtained directly from the oscilloscope and is measured in seconds. The period is measured from
the oscilloscope simply because the value tends to be more accurate at shorter timescales, at least for the
Rigol DS1054Z.

### Setup
Ensure that Rust is installed. Then run:
```
make build
```
This command will run `cargo` under the hood and compile a binary named `nos` to `target/debug`.

## Hardware
### Usage
Out of the box, a circuit board and two leads ("minigrabbers") will be provided:

<p align="center">
  <img width="400" src=docs/preassembled.png>
</p>

Connect the minigrabbers' female header to the 2-pin male header on the board. Then connect the minigrabbers
to the capacitor being studied. Connect a supply voltage to the 3 pin header (the middle pin is unused). The
recommended voltage is between 4.5 and 16V with an 18V absolute maximum, There is no limitation on current as
the device will only draw as much current as it needs. Last, connect the oscilloscope ground lead to the
ground rail and the probe to the single pin header connected to pin 6. The final assembly should follow:

<p align="center">
  <img width="400" src=docs/assembled.png>
</p>

Upon supplying a voltage, a rectangular wave should trigger the scope:

<p align="center">
  <img src=docs/scope_52nf_example.png>
</p>

Note that 5.270 ms period measured by the scope. This value would be supplied to the binary:
```
$ /path/to/nos --period=5.270 --unit=ms
```
Which would return:
```
Parsed frequency (Hz): 189.75331
Parsed period (s):     0.00527
Capacitance (F):       0.000000053254745
Capacitance (uF):      0.053254746
Capacitance (nF):      53.254745
```
