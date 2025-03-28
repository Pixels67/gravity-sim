# Gravity Simulator

A gravity simulator tech demo, made with Macroquad.

## Building from Source

- [Install Rust](https://www.rust-lang.org/learn/get-started)
- Run the following commands:

```sh
git clone https://github.com/Pixels67/gravity-sim.git
cd gravity-sim
cargo build --release
```

***Note***: To get debug lines build without `--release` \
***Note***: On Linux you need to install `libasound2-dev` to build the project, on Debian-Based distros run the following command:
```sh
sudo apt-get install libasound2-dev
```

## Controls

### Movement

**[A]** / **[D]**: Left / Right \
**[W]** / **[S]**: Forward / Back \
**[LShift]** / **[LCtrl]**: Up / Down

### Spawn / Destroy Objects

**[LMB]** To enter object creation mode or if in object creation mode go into drag mode \
**[ESC]** To return to idle/default mode \
**[R]** On an object to remove it \
**[E]** To raise placement or velocity line elevation \
**[Q]** To lower placement or velocity line elevation \
**[UP]** To increase placed object mass \
**[DOWN]** To decrease placed object mass
