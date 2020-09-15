#[macro_use]
extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;

use kiss3d::light::Light;
use kiss3d::window::Window;

use na::Point3;

use std::io;
use std::env::args;
use std::thread;

mod solvestring;
use crate::solvestring::solvestring::solve_string;

fn main(){
    print!("not functional, go home")
}

fn replace(current: f32, equRaw: String) -> String {
    let equNew:String;
    let equRaw2 = &equRaw;
    let replacewith:String = current.to_string();
    if contains(equRaw.to_string(), 'x'.to_string())!=-1{
        equNew = equRaw.replace('x', &replacewith);
    }else if contains(equRaw2.to_string(), 'X'.to_string())!=-1{
        equNew = equRaw.replace('X', &replacewith);
    }else{
        equNew = equRaw;
    }
    return equNew;
}

fn contains(region: String, target: String) -> i32 {
    let mut value:i32 = -1;
    for i in 0..region.len() {
        let current_let = &region[i..i + 1];
        if current_let == target {
            value = i as i32;
            break;
        }
    }
    return value;
}

fn repeater(equation: String, min: String, max: String, rate: String, x: &mut Vec<f32>, y: &mut Vec<f32>, z: &mut Vec<f32>){
    let mut i:f32 = min.parse().unwrap();
    let fmax:f32 = max.parse().unwrap();
    let frate:f32 = rate.parse().unwrap();
    let equRaw = equation;
    let mut equNew:String;
    let mut result:f32;
    while i <= fmax {
        equNew = replace(i, equRaw.to_string());
        x.push(i);
        println!("1: Solving {}",equNew);
        result = solve_string((equNew).to_string()) as f32;
        y.push(result);
        z.push(0.0);
        //println!("6: {} gives us {} when put into {}",x[(i/frate) as usize],y[(i/frate) as usize],equRaw);
        /*
        future use. ignore me for now
        if(result == imaginary){
            z.append(imaginary portion)
            y.append(real portion)
        }else{
            z.append(0)
            y.append(result)
        }
        */
        i = i + frate;
    }
}

fn show_alert_message(clicked: Button) {
    let x:Vec<f32> = vec![];
    let y:Vec<f32> = vec![];
    let z:Vec<f32> = vec![];


    //passthrough(equ.to_string(), min.to_string(), max.to_string(), rate.to_string(), x, y, z);
}
//declares and passes variables. Written By partner
fn passthrough(equation: String, min: String, max: String, rate: String, mut x1: Vec<f32>, mut y1: Vec<f32>, mut z1: Vec<f32>){
    let x2 = &mut x1;
    let y2 = &mut y1;
    let z2 = &mut z1;
    repeater(equation, min, max, rate, x2, y2, z2);
    graph(&x1, &y1, &z1);
    println!("7: Finished Graph")
}

fn graph(x1:&Vec<f32>, y1:&Vec<f32>, z1:&Vec<f32>){
    let mut corda;
    let mut cordb;
    let boxx = [0.0, 0.0, 0.0, 0.0, 0.0, 100.0, -100.0];
    let boxy = [0.0, 100.0, 0.0, -100.0, 0.0, 0.0, 0.0];
    let boxz = [0.0, 0.0, 100.0, 0.0, -100.0, 0.0, 0.0];
    let mut window = Window::new("Kiss3d: lines");
    let look = Point3::new(0.0, 1.0, -10.0);
    let start = Point3::new(0.0, 1.0, 0.0);
    let mut camera = kiss3d::camera::FirstPerson::new(look ,start);
    //let manager = EventManager::new();
    camera.set_yaw_step(0.003);
    camera.set_pitch_step(0.003);
    camera.rebind_down_key(Option::from(kiss3d::event::Key::S));
    camera.rebind_left_key(Option::from(kiss3d::event::Key::A));
    camera.rebind_right_key(Option::from(kiss3d::event::Key::D));
    camera.rebind_up_key(Option::from(kiss3d::event::Key::W));
    window.set_light(Light::StickToCamera);
    while window.render_with_camera(&mut camera) {
        //makes reference
        for i in  1..boxx.len(){
            let cx = boxx[i];
            let cy = boxy[i];
            let cz = boxz[i];
            corda = Point3::new(0.0, 0.0, 0.0);
            cordb = Point3::new(cx, cy, cz);
            window.draw_line(&corda, &cordb, &Point3::new(0.0, 0.0, 1.0));
        }
        //makes graph
        for i in  1..x1.len(){
            let ax = x1[i - 1];
            let ay = y1[i - 1];
            let az = z1[i - 1];
            let bx = x1[i];
            let by = y1[i];
            let bz = z1[i];
            corda = Point3::new(ax, ay, az);
            cordb = Point3::new(bx, by, bz);
            window.draw_line(&corda, &cordb, &Point3::new(1.0, 0.0, 0.0));
        }
    }
}