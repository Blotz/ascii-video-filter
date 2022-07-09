use rscam::{Camera, Config};
use termion;
// use std::{thread, time};

fn main() {
    // Set assci character arr
    const ASCII_ARR: &[u8] = b" .\'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    const ASCII_CONST: f64 = (ASCII_ARR.len() - 1) as f64 / 255.0;
    // Set camera config
    const CAMARA_WIDTH: u32 = 1280;
    const CAMARA_HEIGHT: u32 = 720;
    const CAMARA_FRAME_RATE: u32 = 30;
    // const CAMARA_SLEEP_MS: u64 = ((1.0/30 as f64) * 1000.0) as u64;
    // const CAMARA_DUR: time::Duration = time::Duration::from_millis(((1.0/30 as f64) * 1000.0) as u64);
    const CAMARA_ASPECT_RATIO: f64 = CAMARA_HEIGHT as f64 / CAMARA_WIDTH as f64;

    // Start camara
    let mut camera: Camera = Camera::new("/dev/video0").unwrap();
    // set camara config
    camera.start(&Config {
        interval: (1, CAMARA_FRAME_RATE),      // 30 fps.
        resolution: (CAMARA_WIDTH, CAMARA_HEIGHT),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();
    
    loop {
        // let start = time::Instant::now();

        // Get size of terminal window
        let size = termion::terminal_size().unwrap();
        let terminal_width: u32 = size.0 as u32; 
        let terminal_height: u32 = size.1 as u32;

        // STEP 1: Cal max size of image
        // calcuelate largest image that fits in terminal window
        let width: u32;
        let height: u32;
        let scale_factor: f64;

        // Work out aspect ratio of terminal
        let terminal_aspect_ratio: f64 = terminal_height as f64 / terminal_width as f64;
        
        // height / width = ratio
        // width2 * ratio = height2
        // height2 / ratio = width2
        if terminal_aspect_ratio > CAMARA_ASPECT_RATIO {
            // camara width is limiting factor
            width = terminal_width;
            height = (terminal_width as f64 * CAMARA_ASPECT_RATIO) as u32;
            scale_factor = terminal_width as f64 / CAMARA_HEIGHT as f64;
        } else {
            // camara height is limiting factor
            width = (terminal_height as f64 / CAMARA_ASPECT_RATIO) as u32;
            height = terminal_height;
            scale_factor = terminal_height as f64 / CAMARA_HEIGHT as f64;
        }

        // Step 2: calcuelate average pixel brightness
        // Grab camara input
        let frame = camera.capture().unwrap();
        let mut output = String::new();
        
        // for each pixel in screen, calcuelate average
        for y in 0..height {
            for x in 0..width {
                // range of sample pixels
                let y_start: u32 = ((y as f64 ) / scale_factor) as u32;
                let y_end: u32 = ((y as f64 + 1.0) / scale_factor) as u32;
                let x_start: u32 = ((x as f64 * 3.0) / scale_factor) as u32; // mult by 3 because color = RGB
                let x_end: u32 = (((x+1) as f64 * 3.0) / scale_factor) as u32;

                // calculate total brightness
                // data = [r,g,b, r,g,b,   r,g,b r,g,b, ... ]
                // frame_y * CAMARA_WIDTH + x_start  -  frame_y * CAMARA_WIDTH + x_end [123,123,123]
                let mut brightness: u32 = 0;

                for frame_y in y_start..y_end {
                    let frame_ptr_start = (frame_y * CAMARA_WIDTH*3 + x_start) as usize;
                    let frame_ptr_end = (frame_y * CAMARA_WIDTH*3 + x_end) as usize;

                    let data = &frame[frame_ptr_start..frame_ptr_end];
                    for pixel in data {
                        brightness += *pixel as u32;
                    }
                }

                // STEP 3: convert brightness into ascii image
                brightness = brightness / ((y_end - y_start) * (x_end - x_start));
                let character: char = ASCII_ARR[(brightness as f64 * ASCII_CONST) as usize] as char;
                output.push(character);
                output.push(character);

            }
            output.push('\n');
        }
        println!("{}", output);
        // println!("{:?}", start.elapsed());
        // thread::sleep(CAMARA_DUR - start.elapsed());
    }


    // println!("{} {} {}", CAMARA_WIDTH, CAMARA_HEIGHT, CAMARA_ASPECT_RATIO);
    // println!("{} {} {}", terminal_width, terminal_height, terminal_aspect_ratio);
    // println!("{} {} {}", width, height, scale_factor);
    
}
