
// Add SDL2 library 
use sdl2::{pixels::Color, rect::Rect, video::Window};
// Standard library 
use std::time::Duration;

//  Add dependencies by going to Cargo.toml left side of
//  The window, under EXPLORER > Cargo.toml 
//  Under [dependencies] add, sdl2 = "0.35.1"


// Our main function 
fn main() {

    // Use sdl library to create 
    // sdl stands for simple direct media layer
    let sdl_context = sdl2::init().unwrap();
    // Create a video sub system 
    let video_subsystem = sdl_context.video().unwrap();
    let mut window = video_subsystem
        .window("Opportunity Arena", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas to draw on 
    let mut canvas = window.into_canvas().build().unwrap();

    // Horizontal wall 1
    // Create a width for our wall
    let width: u32 = 750; 
    // create a height for our wall 
    let height: u32 = 10; 
    // Create a square for our wall 
    let square = Rect::new(25, 100, width, height);
    // Set a color for our canvas
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    // Fill up the square
    canvas.fill_rect(square).unwrap();

    //=================================================================================
   
         // Horizontal wall 2 

         let width2: u32 = 750;
         let height2: u32 = 5;
         let square2 = Rect::new(20,50,width2,height2);
         canvas.set_draw_color(Color::RGB(151, 94, 167));
         canvas.fill_rect(square2).unwrap();


    // Draw a dot in the center of the square
    // Create a variable for our dot size
    let dot_size: u32 = 10;
    // Create a dot structure that contains the position and size of our dot 
    let dot_rect = Rect::new(
        width as i32,
        150 as i32,
        dot_size, 
        dot_size,
    );

    // Set the draw color, and we want to do this everytime we draw something. 
    canvas.set_draw_color(Color::RGB(0,255,255));
    // fill the dot
    canvas.fill_rect(dot_rect).unwrap();

    //=================================================================================
    // Draw a dot in the center of the square
    // Create a var for our dot size
    let dot2_size: u32 = 25;
    // Create a dot structure that contains the position and size of our dot
    let dot_rect = Rect::new(255 as i32, 
        550 as i32, 
        dot2_size, 
        dot2_size
    );

    // Set the draw color, and we want to do this everytime we draw something. 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    // fill the dot 
    canvas.fill_rect(dot_rect).unwrap();
    //======================================================================================
    
    let sqr3_size: u32 = 35;
    let dot_rect3 = Rect::new(200 as i32, 
        450 as i32, 
        sqr3_size,
        sqr3_size);
    
    canvas.set_draw_color(Color::RGB(151, 94, 167));
    canvas.fill_rect(dot_rect3).unwrap();


    let sqr4_size: u32 = 35;
    let dot_rect4 = Rect::new(220 as i32, 
        300 as i32, 
        sqr4_size,
        sqr4_size);
    
    canvas.set_draw_color(Color::RGB(215, 94, 167));
    canvas.fill_rect(dot_rect4).unwrap();


    let sqr5_size: u32 = 35;
    let dot_rect5 = Rect::new(100 as i32, 
        200 as i32, 
        sqr5_size,
        sqr5_size);
    
    canvas.set_draw_color(Color::RGB(37, 76, 181));
    canvas.fill_rect(dot_rect5).unwrap();


    let sqr6_size: u32 = 35;
    let dot_rect6 = Rect::new(600 as i32, 
        300 as i32, 
        sqr6_size,
        sqr6_size);
    
    canvas.set_draw_color(Color::RGB(154, 180, 88));
    canvas.fill_rect(dot_rect6).unwrap();


    // Draw the canvas on the screen
    canvas.present();

    // Loop until the user closes the window by pressing escape
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                sdl2::event::Event::Quit{ .. } |
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
            std::thread::sleep(Duration::from_millis(16));
        }
    }



