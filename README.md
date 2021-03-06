# USE [OpenRGB](https://gitlab.com/CalcProgrammer1/OpenRGB) INSTEAD

This project is not actively maintained. No support for additional boards will be added.

At the time of writing OpenRGB is actively maintained and supports variety of devices, including
MSI boards with code adapted from this project. Just use that. If it doesn’t work, this tool won’t
work either. If it doesn’t support functionality this tool does, consider porting similar
functionality to OpenRGB.

---

Utility for controlling RGB header on MSI boards

[How this utility came to be](http://kazlauskas.me/entries/i-reverse-engineered-a-motherboard.html)

This utility not only works on any linux system you find around, it also is much more flexible than
the 7 colours MSI’s own Gaming App. Futhermore, unlike the MSI’s utility, this does not make your
system vulnerable to anybody who cares to fiddle around the system.

* Linux (/dev/port, might work on WSL?) or FreeBSD (/dev/io);
* Only MSI motherboards with NCT6795D super I/O chip;
  * Run a recent version of sensors-detect to check if you have this chip;
* No warranty whatsoever (read the license);
  * If you find your board misbehaving, try clearing CMOS;

# Reportedly Working boards

* B350 MORTAR ARCTIC
* B350 PC MATE
* B350 TOMAHAWK
* B360M GAMING PLUS
* B450 GAMING PLUS AC
* B450 MORTAR
* B450 TOMAHAWK
* H270 MORTAR ARCTIC
* H270 TOMAHAWK ARCTIC
* X470 GAMING PLUS
* X470 GAMING PRO
* Z270 GAMING M7
* Z270 SLI PLUS
* Z370 MORTAR
* Z370 PC PRO
* Z270 GAMING M6 AC (confirmed to correctly disable LEDs only)
* B450M Mortar Titanium (with [some caveats](https://github.com/nagisa/msi-rgb/issues/119))
* B360 gaming plus ([colours inverted](https://github.com/nagisa/msi-rgb/issues/118))
* Z390 Gaming Plus ([#116](https://github.com/nagisa/msi-rgb/issues/116))
* b450m pro-vdh max ([#114](https://github.com/nagisa/msi-rgb/issues/114), [#109](https://github.com/nagisa/msi-rgb/issues/109))
* B450M GAMING PLUS ([#112](https://github.com/nagisa/msi-rgb/issues/112))

# How to compile and run

To compile this project you’ll need rustc and cargo. Get them at your package manager or
[here](https://www.rust-lang.org/en-US/install.html).

Then:

```
git clone https://github.com/nagisa/msi-rgb
cd msi-rgb
cargo build --release
```

You’ll need root to run this program:

```
sudo ./target/release/msi-rgb 00000000 FFFFFFFF 00000000 # for green
```

The hexa numbers represent each color as a sequence *in time* per byte so 4 change of colors.

```
sudo ./target/release/msi-rgb FF000000 00FF0000 0000FF00 # this makes red then green then blue then off then red etc..
```

Run following for more options:

```
./target/release/msi-rgb -h
```

# Examples

## Heartbeat

```
sudo ./target/release/msi-rgb 206487a9 206487a9 10325476 -ir -ig -ib -d 5
```

[![animation of pulse](https://thumbs.gfycat.com/BlueWhichAntbear-size_restricted.gif)](https://gfycat.com/BlueWhichAntbear)

## Police

```
sudo ./target/release/msi-rgb -d15 FF00FF00 0 00FF00FF
```

[![animation of police](https://thumbs.gfycat.com/RemoteChiefBobolink-size_restricted.gif)](https://gfycat.com/RemoteChiefBobolink)

## Happy Easter

[From colourlovers](http://www.colourlovers.com/palette/4479254/Happy-Easter-2017!)

```
sudo ./target/release/msi-rgb 58e01c0d 504fdcb9 e4aa75eb --blink 2 -d 32
```

[![animation of happyeaster](https://thumbs.gfycat.com/DirectBleakBuzzard-size_restricted.gif)](https://gfycat.com/DirectBleakBuzzard)

## Hue wheel (t HUE, 0.9 SATURATION, 1.0 VALUE) (REQUIRES PYTHON)

![animation of hue wheel](https://thumbs.gfycat.com/ViciousGreenBittern-size_restricted.gif)

```
echo -e "import colorsys, time, subprocess\ni=0\nwhile True:\n  subprocess.call(['target/release/msi-rgb', '-d511'] + list(map(lambda x: ('{0:01x}'.format(int(15*x)))*8, colorsys.hsv_to_rgb((i % 96.0) / 96.0, 0.9, 1))))\n  time.sleep(0.1)\n  i+=1" | sudo python -
```

# Implementation

For implementation details, including the registers used by super I/O and their meanings see the
comment in the `src/main.rs` file.

# License

Code is licensed under the permissive ISC license. If you create derivative works and/or nice RGB
schemes, I would love to see them :)
