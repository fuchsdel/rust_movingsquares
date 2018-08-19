extern crate piston_window;
extern crate rand;
use piston_window::*;

const WINDOWSIZE: (f64, f64) = (500.0, 500.0);
const NUMRECT: usize = 100;

struct Testrect{
    color:[f32;4],
    colorvec:[f32;3],
    //it has been shown that this has to be [f64;4] !!
    pos:[f64;4],
    veloc:(f64, f64),
}

impl Testrect{
    //Function to update a given colored rect:
    fn update(&mut self){
        //update color of the rect
        for i in 0..3{
            self.color[i]-=self.colorvec[i];
            if self.color[i] < 0.0{
                self.color[i] = 1.0;
            }
        }
         //Is the same as ^^^^^^^^
         //updatecolor(&mut self.color[0]);
         //updatecolor(&mut self.color[1]);
         //updatecolor(&mut self.color[2]); 
         

        //update position:
        self.pos[0] += self.veloc.0;
        self.pos[1] += self.veloc.1;
        //check if position is valid:
        if self.pos[0] < 0.0 ||
           self.pos[1] <0.0 ||
           self.pos[0]+self.pos[2] > WINDOWSIZE.0 ||
           self.pos[1]+self.pos[3] > WINDOWSIZE.1 
           {
                self.veloc.0 *= -1.0;
                self.veloc.1 *= -1.0;
                if self.pos[0] < 0.0 {self.pos[0] = 0.0;}
                if self.pos[1] < 0.0 {self.pos[1] = 0.0;}
                if self.pos[0]+self.pos[2] > WINDOWSIZE.0{ self.pos[0] = WINDOWSIZE.0-self.pos[2];}
                if self.pos[1]+self.pos[3] > WINDOWSIZE.1{ self.pos[1] = WINDOWSIZE.1-self.pos[3];}  
            }
                   
    }
}

fn fill_array_with_random_values(array:&mut [f32], min: f32, max:f32){
    let diff = max-min;
    for i in array.iter_mut(){
        *i = rand::random::<f32>()*diff+min;
    }
}

fn fill_array_with_random_valuesf64(array:&mut [f64], min: f64, max:f64){
    let diff = max-min;
    for i in array.iter_mut(){
        //println!("Before err:{}",i);
        *i = rand::random::<f64>()*diff+min;
    }
}

fn create_random_colorrects()->Vec<Testrect>{
    let mut thearr: Vec<Testrect> = Vec::new();
    for i in 0..NUMRECT{
        thearr.push(Testrect{
            color:[0.0,0.0,0.0,1.0],
            pos:[0.0,0.0,64.0,64.0],
            veloc:(0.3,0.4),
            colorvec:[0.001,0.002,0.05]
        });
        //fill with random values!
        //fill_array_with_random_valuesf64(&mut thearr[i].pos[2..], 0.0, 128.0);
        fill_array_with_random_values(&mut thearr[i].colorvec, 0.001, 0.05);
        fill_array_with_random_valuesf64(&mut thearr[i].pos[2..], 64.0, 128.0);
        thearr[i].pos[0] = rand::random::<f64>()*(WINDOWSIZE.0-thearr[i].pos[1])+thearr[i].pos[1];
        thearr[i].pos[1] = rand::random::<f64>()*(WINDOWSIZE.0-thearr[i].pos[2])+thearr[i].pos[2];        
        thearr[i].veloc.0 = rand::random::<f64>()*0.5+0.3;
        thearr[i].veloc.1 = rand::random::<f64>()*0.5+0.3;
        //println!("Color of rect {}:\t{:?}; pos:{:?}",i,thearr[i].color,thearr[i].pos);
    }
    thearr 
}

fn main() {
    //our first init testrect!
    let mut myrect = Testrect{
        color:[0.0,0.0,0.0,1.0],
        pos:[0.0,0.0,64.0,64.0],
        veloc:(0.3,0.4),
        colorvec:[0.001,0.002,0.05]
    };

    let mut my_vec_rects = create_random_colorrects();

    //init the screen
    let mut window: PistonWindow = WindowSettings::new("TestPiston!",[WINDOWSIZE.0 as u32, WINDOWSIZE.1 as u32])
    .exit_on_esc(true)
    .vsync(true)
    .build()
    .unwrap();

    //the background color white = [1.0; 4]
    let bg = [0.0;4];

    while let Some(e) = window.next(){
        match e{
            Input::Render(_)=>{
                window.draw_2d(&e,|c,g|{
                    clear(bg,g);
                    rectangle(
                        myrect.color,
                        myrect.pos,
                        c.transform,
                        g
                    );

                    for rect in my_vec_rects.iter(){
                        rectangle(
                            rect.color,
                            rect.pos,
                            c.transform,
                            g
                        );
                    }
                });
            }
            Input::Update(_)=>{
                myrect.update();
                //myrect.update
                for rect in my_vec_rects.iter_mut(){
                    rect.update()
                }
            }
            _ => {},
        }
    }
}
