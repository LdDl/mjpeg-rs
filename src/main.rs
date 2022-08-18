use opencv::{
    prelude::*,
    videoio::VideoCapture,
    videoio::CAP_ANY,
    core::Mat,
    highgui::named_window,
    highgui::resize_window,
    highgui::imshow,
    highgui::wait_key,
};

use std::thread;
use std::sync::{mpsc};
mod mjpeg_streaming;
use mjpeg_streaming::start_mjpeg_streaming;

fn run(src: String) -> opencv::Result<()> {
    println!("Video source is: {src}");
    let mut video_capture = VideoCapture::from_file(&src, CAP_ANY)?;
    let mut first_frame = Mat::default();
    /* Read first frame to determine image width/height */
    video_capture.read(&mut first_frame)?;
    let first_frame_cols = first_frame.cols() as u32;
    let first_frame_rows = first_frame.rows() as u32;

    let window_name = "imshow win";
    let window = named_window(window_name, 1)?;
    resize_window(window_name, 640, 380)?;
    let (tx, rx) = mpsc::sync_channel(25);
    let (tx_mjpeg, rx_mjpeg) = mpsc::sync_channel(25);

    thread::spawn(move || {
        match start_mjpeg_streaming("localhost".to_string(), 8090, rx_mjpeg, first_frame_cols, first_frame_rows) {
            Ok(_) => {},
            Err(err) => {
                panic!("Can't start MJPEG streaming due the error: {:?}", err)
            }
        }
    });

    thread::spawn(move || {
        loop {
            let mut read_frame = Mat::default();
            match video_capture.read(&mut read_frame) {
                Ok(_) => {},
                Err(_) => {
                    println!("Can't read next frame");
                    break;
                }
            }
            let mut mjpeg_frame = unsafe {
                Vec::from(std::slice::from_raw_parts(
                    read_frame.data() as *const u8,
                    (first_frame_cols * first_frame_rows * 3) as usize,
                ))
            };
            for i in 0..(first_frame_cols * first_frame_rows) {
                mjpeg_frame.swap((i * 3) as usize, (i * 3 + 2) as usize);
            }
            tx_mjpeg.send(mjpeg_frame).unwrap();
            tx.send(read_frame).unwrap();
        }
    });
    
    for received in rx {
        let mut cloned_frame = received.clone();
        imshow(window_name, &mut cloned_frame).unwrap();
        let key = wait_key(10).unwrap();
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    let video_src_path = "./data/sample_960_540.mp4".to_string();
    run(video_src_path).unwrap()
}