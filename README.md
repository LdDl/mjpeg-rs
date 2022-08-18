# MJPEG server in Rust

## Table of Contents

- [About](#about)
- [Usage](#usage)
- [Credits](#credits)

## About

Just a template code for serving MJPEG stream via OpenCV.

Yes, this repo code is stolen (*highly insipred* if you want to) from https://github.com/dskkato/mjpeg-rs and in https://github.com/t924417424/mjpeg_rs.

What the purpose then? Well, I've change a couple of things:

* Eliminate use of [image](https://crates.io/crates/image) crate for encoding purposes. Is slows down MJPEG streaming for me drastically. I use [imencode](./src/main.rs#L59) instead.
* Separated thread for MJPEG streaming (proof of concept)
* Bump to latest [actix-web](https://actix.rs/) web framework

## Usage

Just modify Rust code in [main](src/main.rs) file to adjust your needs:

```rust
    // ...
    // Path to video (could be rtsp or local video file)
    let video_src_path = "./data/sample_960_540.mp4".to_string();
    // ...
    // ...
    // Change host and port for live streaming if needed
    start_mjpeg_streaming("localhost".to_string(), 8090, rx_mjpeg, first_frame_cols, first_frame_rows) {
    // ...
```

Start:

```
cargo run
```

Open http://localhost:8090/live in browser and enjoy

There would be opened imshow() object also (don't close it accidenlty since main thread will be terminated).

## Credits
Thanks to:
 * https://github.com/t924417424/mjpeg_rs for inspiring
 * https://github.com/dskkato/mjpeg-rs for actix-web based version
 * https://actix.rs/ for great Web-framework
 * https://github.com/twistedfall/opencv-rust for binding to [OpenCV](https://opencv.org/)
 * Rust community and not mentioned authors of other dependencies