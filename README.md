Utility for controlling RGB header on MSI boards

This utility not only works on any linux system you find around, it also is much more flexible than
the 7 colours MSI’s own Gaming App. Futhermore, unlike the MSI’s utility, this does not make your
system vulnerable to anybody who cares to fiddle around the system.

See this blog (link incoming later) post for details on how this app came to be

* Only UNIX where /dev/port matches the behaviour of Linux’s /dev/port (might work on WSL?);
* Only MSI motherboards with NCT6795D super I/O chip;
  * Run a recent version of sensors-detect to check if you have this chip;
* No warranty whatsoever (read the license);
  * If you find your board misbehaving, try clearing CMOS;

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

# License

Code is licensed under the permissive ISC license. If you create derivative works and/or nice RGB
schemes, I would love to see them :)
