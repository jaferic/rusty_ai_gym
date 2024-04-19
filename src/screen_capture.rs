use crate::input_capture;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time::Duration, time::Instant};
use xcap::Window;

fn select_window() -> Option<Window> {
    let windows = Window::all().unwrap();
    println!("please select a window to capture:");
    for (index, window) in windows.iter().enumerate() {
        if window.is_minimized() {
            continue;
        }
        println!(
            "{}: {} ({}x{})",
            index,
            window.title(),
            window.width(),
            window.height()
        );
    }
    println!("Enter the number of the window:");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().expect("invalid input!");
    windows.into_iter().nth(choice)
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub fn screen_capture() {
    let start = Instant::now();
    let capture_interval = Duration::from_millis(100);
    let mut frame_count = 0;

    //loop {
    if let Some(window) = select_window() {
        let mut image = window.capture_image().unwrap();
        loop {
            let actions = vec!["a", "s", "d", "w"];
            let random_action = actions.choose(&mut rand::thread_rng());
            match random_action {
                Some(action) => println!("Random action: {}", action),
                None => println!("The actions list is empty."),
            }
            input_capture::input_capture(*action);
            image
                .save(format!(
                    "target/window-{}-{}.png",
                    frame_count,
                    normalized(window.title())
                ))
                .unwrap();
            frame_count += 1;
            let elapsed = start.elapsed();
            println!("Time taken for capturing: {:?}", elapsed);
            if elapsed < capture_interval {
                thread::sleep(capture_interval - elapsed);
            }
            println!(
                "frame window-{:?}-{:?} matches this input {:?}",
                frame_count,
                normalized(window.title()),
                action
            );
            image = window.capture_image().unwrap();
        }
    } else {
        println!("No window selected or invalid");
    }
    //}

    println!("Capturing: {:?}", start.elapsed());
}

// Convert the BGRA frame to RGBA
//let mut bitflipped = Vec::with_capacity(frame.len());
