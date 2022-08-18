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

fn run(src: String) -> opencv::Result<()> {
    println!("Video source is: {src}");
    let mut video_capture = VideoCapture::from_file(&src, CAP_ANY)?;
    let window_name = "imshow win";
    let window = named_window(window_name, 1)?;
    resize_window(window_name, 640, 380)?;
    let (tx, rx) = mpsc::sync_channel(25);
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