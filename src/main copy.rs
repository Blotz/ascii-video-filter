use rscam::{Camera, Config};
use termion;

fn main() {
    // Set assci character arr
    const ASCII_ARR: &[u8] = b" .,-:;i1tfL@#";
    const CAMARA_WIDTH: u32 = 1280;
    const CAMARA_HEIGHT: u32 = 720;
    const CAMARA_FRAME_RATE: u32 = 30;

    // Start camara
    let mut camera = Camera::new("/dev/video0").unwrap();
    // set camara config
    // use gray scale
    camera.start(&Config {
        interval: (1, CAMARA_FRAME_RATE),      // 30 fps.
        resolution: (CAMARA_WIDTH, CAMARA_HEIGHT),
        format: b"RGB3",
        ..Default::default()
    }).unwrap();
    

    // Get size of terminal window
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    // Get frame
    let frame = camera.capture().unwrap();
    // resize frame
    let width_ratio: u32 = (CAMARA_WIDTH / terminal_width as u32) * 3;
    let height_ratio: u32 = CAMARA_HEIGHT / terminal_height as u32;
    // create a buffer to store the frame
    let mut buffer = vec!['#'; (terminal_width * terminal_height) as usize];
    // print!("{:?}", &frame[..]);

    for n in 0..buffer.len() {
        // for each pixel in buffer
        // jump ratio pixels in frame
        // width and height here are the frame pixel locations
        let start_width: u32  = n as u32 * width_ratio;
        let end_width: u32    = (n as u32 + 1) * width_ratio;
        let start_height: u32 = n as u32 * height_ratio;
        let end_height: u32   = (n as u32 + 1) * height_ratio;

        let mut brightness: u32 = 0;

        for y in start_height..end_height {
            let x: u32 = y*CAMARA_WIDTH + start_width;
            let x_end: u32 = y*CAMARA_WIDTH + end_width;
            // calcuelate sum of pixel values
            for x in x..x_end {
                brightness += frame[x as usize] as u32;
            }
        }

        // calculate average brightness
        brightness = brightness / (height_ratio) * (width_ratio);

        // print!("{}", ASCII_ARR[brightness_index]);
        buffer[n] = ASCII_ARR[brightness as usize] as char;
    }
    

    // print buffer to console
    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::Goto(1, 1));
    print!("{}", termion::cursor::Hide);
    // print array
    // print!("{:?}", buffer);
    // loop though width and height of buffer
    for y in 0..terminal_height {
        for x in 0..terminal_width {
            print!("{}", buffer[(y * terminal_width + x) as usize]);
        }
    }
    

}
