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

# Working boards

This is a list of reportedly working motherboards. If the tool works on your motherboard and it is
not listed here, consider filling an issue or writing me an email and I’ll add it here.

* B350 MORTAR ARCTIC
* B350 TOMAHAWK
* X470 GAMING PRO
* Z270 SLI PLUS
* H270 TOMAHAWK ARCTIC

If your board is not working, and your motherboard is not [on this
list](https://github.com/nagisa/msi-rgb/issues?q=is%3Aissue+is%3Aopen+label%3Aboard), a new issue
would be greatly appreciated.

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

## Hue wheel (t HUE, 0.9 SATURATION, 1.0 VALUE)

![animation of hue wheel](https://thumbs.gfycat.com/ViciousGreenBittern-size_restricted.gif)

```
echo -e "import colorsys, time, subprocess\ni=0\nwhile True:\n  subprocess.call(['target/release/msi-rgb', '-d511'] + map(lambda x: ('{0:01x}'.format(int(15*x)))*8, colorsys.hsv_to_rgb((i % 96.0) / 96.0, 0.9, 1)))\n  time.sleep(0.1)\n  i+=1" | sudo python -
```

# Implementation

For implementation details, including the registers used by super I/O and their meanings see the
comment in the `src/main.rs` file.

# License

Code is licensed under the permissive ISC license. If you create derivative works and/or nice RGB
schemes, I would love to see them :)
