use raylib;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use std::time::Instant;

use rust_math::trigonometry;

const SCREEN_WIDTH: f32 = 900.0;
const SCREEN_HEIGHT: f32 = 600.0;

const GRAVITY: f32 = -0.0;

struct Particle {
    position: Vector2,
    height: f32,
    velocity: Vector2,
    color: Color,
    charge: f32,
    mass: f32
}

fn main() {
    //Working variables
    let time = Instant::now();
    let mut time_str: &str;

    //Objects
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Project Particles").vsync().build();

    
    //Particles
    let mut particle1 = Particle {
        position: Vector2::new((SCREEN_WIDTH / 2.0)-50.0,(SCREEN_HEIGHT / 2.0) -100.0),
        height: 0.0,
        velocity: Vector2::new(0.0, 0.0),
        color: Color::BLUE,
        charge: -10.0,
        mass: 1.0
    };
    let mut particle2 = Particle {
        position: Vector2::new((SCREEN_WIDTH / 2.0)+50.0,(SCREEN_HEIGHT / 2.0) +100.0),
        height: 0.0,
        velocity: Vector2::new(0.0, 0.0),
        color: Color::RED,
        charge: 10.0,
        mass: 0.0
    };

    //Main loop
    while !rl.window_should_close() {
        //Close Program if user presses escape
        if rl.is_key_down(KEY_ESCAPE) {break;}

        //Calculate distance between both particles
        let horizontal_distance: f32=particle2.position.x-particle1.position.x;
        let vertical_distance: f32=particle2.position.y-particle1.position.y;

        let distance = f32::sqrt((horizontal_distance*horizontal_distance)+(vertical_distance*vertical_distance));

        //force's angle
        //let electric_force_angle: f32 = trigonometry::cosec(vertical_distance*(trigonometry::sin(90.0)/distance));

        //let mut electric_force: f32 = (particle1.charge*particle2.charge)/(distance*distance);

        //let horizontal_electric_force: f32= /*electric_force*trigonometry::cos(20.0)*/ -0.00187938524;
        //let vertical_electric_force: f32= /*electric_force*trigonometry::sin(20.0)*/ -0.00068404028;

        //let horizontal_electric_acceleration: f32 = horizontal_electric_force/particle1.mass;
        //let vertical_electric_acceleration: f32 = vertical_electric_force/particle1.mass;

        //particle1.acceleration.x=horizontal_electric_acceleration;
        //particle2.acceleration.x=horizontal_electric_acceleration;

        //particle1.acceleration.y+=vertical_electric_acceleration;
        //particle2.acceleration.y-=vertical_electric_acceleration;

        //Action
        particle1.position.y += particle1.velocity.y;
        particle1.position.x += particle1.velocity.x;

        particle2.position.y += particle2.velocity.y;
        particle2.position.x += particle2.velocity.y;

        //Calculate height
        particle1.height=SCREEN_HEIGHT-particle1.position.y;
        particle2.height=SCREEN_HEIGHT-particle2.position.y;

        //Attraction
        particle1.position.x += particle1.velocity.x;
        particle2.position.x += particle2.velocity.x;

        //Bounce
        if particle1.height < 5.0 && particle1.velocity.y > -0.5 {
            particle1.velocity.y *= -1.0;
        }
        if particle1.height < 5.0 && particle1.velocity.y > 0.5 {
            particle1.velocity.y = 0.0;
        }


        if particle2.height < 5.0 {
            particle2.velocity.y *= -1.0;
        }

        //G acceleration??
        particle1.velocity.y -= -0.00978885438;
        particle2.velocity.y += -0.00978885438;

        //Electric acceleration
        particle1.velocity.x -= -0.00089442719;
        particle2.velocity.x -= -0.00089442719;

        //Collisions
        if (particle1.position.x <= particle2.position.x + (10.0)) && 
        particle1.position.x >= particle2.position.x - (10.0) && 
        particle1.position.y <= particle2.position.y + (10.0) && 
        particle1.position.y >= particle2.position.y - (10.0){
            //electric_force /= 10.0;
        }

        println!("{}", particle1.velocity.y);

        let time_str = format!("time: {}", (time.elapsed().as_millis() as f64)/1000.0);

        //Rendering
        let mut window=rl.begin_drawing(&thread);
        window.draw_text(&time_str, 12, 12, 20, Color::WHITE);
        window.draw_line_v(Vector2::new(0.0, SCREEN_HEIGHT), Vector2::new(SCREEN_WIDTH, SCREEN_HEIGHT), Color::WHITE);
        window.draw_circle_v(particle1.position, 5.0, particle1.color);
        window.draw_circle_v(particle2.position, 5.0, particle2.color);
        window.clear_background(Color::BLACK);
    }
}