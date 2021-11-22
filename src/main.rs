extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const RED: u32 = 65536*255 + 256*0 + 0;
const GREEN: u32 = 65536*0 + 256*255 + 0;
const BLUE: u32 = 65536*0 + 256*0 + 255;
const YELLOW: u32 = 65536*255 + 256*255 + 0;
const CYAN: u32 = 65536*0 + 256*255 + 255;
const MAGENTA: u32 = 65536*255 + 256*0 + 255;
const BLACK: u32 = 0;
const WHITE: u32 = 65536*256-1;

trait Shape {
    fn color_at(&self, u:f32, v:f32, o:&Pattern)->u32;
}

enum Fill {
    Solid(u32),
    Pattern(Box<Pattern>),
}

struct Pattern {
    a: Fill,
    b: Fill,
    shape: Box<dyn Shape>,
}

impl Pattern {
    fn color(&self, u:f32, v:f32)->u32 {
        self.shape.color_at(u, v, self)
    }
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let subp = Pattern {
        a:Fill::Solid(GREEN),
        b:Fill::Solid(YELLOW),
        shape: Box::new(Checker)
    };
    let subp2 = Pattern {
        a:Fill::Solid(0),
        b:Fill::Solid(MAGENTA),
        shape: Box::new(Stripes)
    };
    let p = Pattern {
        a: Fill::Pattern(Box::new(subp)),
        b: Fill::Pattern(Box::new(subp2)),
        shape: Box::new(Rings),
    };

    let conv = WIDTH as f32;
    for y in 0..HEIGHT {
        let v = (y as f32)/conv-0.5;
        for x in 0..WIDTH {
            let u = (x as f32)/conv-0.5;
        
            buffer[y*WIDTH+x]=p.color(u,v);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

 
struct Checker;
struct Stripes;
struct Rings;



impl Shape for Checker {
    fn color_at(&self, u:f32, v:f32, p:&Pattern)->u32 {
        let x = (u*8.0).floor() as i32;
        let y = (v*8.0).floor() as i32;
        if (x+y) % 2 == 0 {
            match &p.a {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
            
        } else {
            match &p.b {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
        }
    
    }
}


impl Shape for Rings {
    fn color_at(&self, u:f32, v:f32, p:&Pattern)->u32 {
        let u1 = u*16.0;
        let v1 = v*16.0;
        let d = (u1 * u1 + v1 * v1).sqrt().floor() as i32;
        if d % 2 == 0 {
            match &p.a {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
            
        } else {
            match &p.b {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
        }
    
    }
}

impl Shape for Stripes {
    fn color_at(&self, u:f32, v:f32, p:&Pattern)->u32 {
        let u1 = u*24.0;
        
        let d = u1.floor() as i32;
        if d % 2 == 0 {
            match &p.a {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
            
        } else {
            match &p.b {
                Fill::Solid(c)=>*c,
                Fill::Pattern(p) => p.as_ref().color(u, v),
            }
        }
    
    }
}


