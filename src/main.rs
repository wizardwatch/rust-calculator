#[macro_use]
extern crate kiss_ui;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate num;

use kiss_ui::prelude::*;
use kiss_ui::container::*;
use kiss_ui::dialog::Dialog;
use kiss_ui::text::*;
use kiss_ui::button::*;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::Point3;
use kiss3d::event::Key::*;
use kiss3d::event::MouseButton::*;
use kiss3d::event::*;

fn main(){
        kiss_ui::show_gui(|| {
            Dialog::new(
                Grid::new(
                    children![
                    Label::new("Enter a message:"),
                    Label::new("Equation:"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("equ_raw"),
                    Label::new("Minimum:"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("min"),
                    Label::new("Maximum:"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("max"),
                    Label::new("Rate:"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("rate"),
                    Button::new()
                        .set_label("Graph")
                        .set_onclick(show_alert_message),
                ]
                )
                    .set_ndiv(3 as u32)

            )
                .set_title("Hello, world!")
                .set_size_pixels(640, 480)
        });
    use std::io;
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("input received")
        },
        Err(e) => println!("input faiwed owO. Pwease repowt to github uwu.")
    }
}

fn show_alert_message(clicked: Button) {
    let x:Vec<f32> = vec![];
    let y:Vec<f32> = vec![];
    let z:Vec<f32> = vec![];
    let dialog = clicked.get_dialog().unwrap();
    let text_box1 = dialog.get_child("equ_raw").unwrap()
        .try_downcast::<TextBox>().ok().expect("child equ_raw was not a TextBox!");
    let equ = text_box1.get_text();
    let text_box2 = dialog.get_child("min").unwrap()
        .try_downcast::<TextBox>().ok().expect("child min was not a TextBox!");
    let min = text_box2.get_text();
    let text_box3 = dialog.get_child("max").unwrap()
        .try_downcast::<TextBox>().ok().expect("child max was not a TextBox!");
    let max = text_box3.get_text();
    let text_box4 = dialog.get_child("rate").unwrap()
        .try_downcast::<TextBox>().ok().expect("child rate was not a TextBox!");
    let rate = text_box4.get_text();
    dialog.hide();
    passthrough(equ.to_string(), min.to_string(), max.to_string(), rate.to_string(), x, y, z);
}

fn passthrough(equation: String, min: String, max: String, rate: String, mut x1: Vec<f32>, mut y1: Vec<f32>, mut z1: Vec<f32>){
    let x2 = &mut x1;
    let y2 = &mut y1;
    let z2 = &mut z1;
    repeater(equation, min, max, rate, x2, y2, z2);
    graph(&x1, &y1, &z1);
    println!("7: Finished Graph")
}

fn repeater(equation: String, min: String, max: String, rate: String, x: &mut Vec<f32>, y: &mut Vec<f32>, z: &mut Vec<f32>){
    let mut i:f32 = min.parse().unwrap();
    let fmax:f32 = max.parse().unwrap();
    let frate:f32 = rate.parse().unwrap();
    let equRaw = equation;
    let mut equNew:String = "owo".to_string();
    let mut result:f32= 0.0;
    while !(i > (fmax)) {
        equNew = replace(i, equRaw.to_string());
        x.push(i);
        println!("1: Solving {}",equNew);
        result = solve_string((equNew).to_string()) as f32;
        y.push(result);
        z.push(0.0);
        i = i + frate;
    }
}
fn replace(current: f32, equRaw: String) -> String {
    let mut equNew:String = "owo".to_string();
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

fn graph(x1:&Vec<f32>, y1:&Vec<f32>, z1:&Vec<f32>){
    let mut corda;
    let mut cordb;
    let boxx = [0.0,0.0, 0.0, 0.0, 0.0];
    let boxy = [0.0,10.0, 0.0, -10.0,0.0];
    let boxz = [0.0,0.0, 10.0, 0.0,-10.0];
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
fn solve_string(mut input:String) -> f32 {
    println!("The original input is {}", input);
    let mut answer:f32 = 404.0;
    let mut operator_places = vec![];
    let exponent = "^";
    let multiplication = "*";
    let division = "/";
    let addition = "+";
    let subtraction = "~";
    let mut current_letter;
    let mut num_of_parth = 0;
    let mut most_inner_parth = 0;
    let mut places_after_most_inner = 0;
    let mut runner = 0;
    // code here
    let before_input = input.clone();
    input = "".to_string();
    input.push_str("(");
    input.push_str(&before_input);
    input.push_str(")");

    for i in 0..input.len() {
        current_letter = &input[i..i + 1];
        if current_letter == "("{
            num_of_parth = num_of_parth +1
        }
    }
    while num_of_parth > 0 {
        most_inner_parth = 0;
        places_after_most_inner = 0;
        runner = 0;
        println!("I repeated num_of_parth");
        for i in 0..input.len() {
            current_letter = &input[i..i + 1];
            if current_letter == "("{
                most_inner_parth = most_inner_parth + runner;
                runner = 0;
            }
            else if current_letter == ")"{
                break;
            }
            runner = runner + 1;
        }
        println!{"most inner_parth = {}", most_inner_parth};
        for i in most_inner_parth..input.len() {
            current_letter = &input[i..i+1];
            println!("ran most inner parth = {}", most_inner_parth);
            if current_letter == ")"{
                break;
            }
            else{
                places_after_most_inner = places_after_most_inner+1;
                println!("iterated places after most inner");
            }
        }
        let mut new_input = (&input[(most_inner_parth+1) as usize..(most_inner_parth +places_after_most_inner) as usize]).to_string();
        for i in 0..new_input.len() {
            current_letter = &new_input[i..i + 1];
            if current_letter == exponent {
                operator_places.push(exponent);
            }
        }
        println!(" This  is the input as of line 247 {}", new_input);
        for i in 0..new_input.len() {
            current_letter = &new_input[i..i + 1];
            if current_letter == multiplication {
                operator_places.push(multiplication);
            } else if current_letter == division {
                operator_places.push(division);
            }
        }
        for i in 0..new_input.len() {
            current_letter = &new_input[i..i + 1];
            if current_letter == addition{
                operator_places.push(addition);
            }

            else if current_letter == subtraction{
                operator_places.push(subtraction);
            }
        }
        for i in 0..operator_places.len() {
            let mut beforeplaces = 1;
            let mut afterplaces = 1;
            let current_operator = operator_places[i];
            let mut int_cop:i32 = 0;
            for s in 0..new_input.len(){
                println!("test 1");
                current_letter = &new_input[s..s + 1];
                if current_operator == current_letter{
                    println!("test 2");
                    let str_cop = s.to_string();
                    int_cop = str_cop.parse::<i32>().unwrap();
                }
            }
            let mut ch_p:usize;
            loop{
                println!(" This  is the input as of line 256 {}", new_input);
                println!("int_cop = {}", int_cop);
                ch_p = (int_cop - beforeplaces) as usize;
                println!("ch_p = {}", ch_p);
                if ch_p == 0{
                    break;
                }
                let ch = &new_input[ch_p-1..ch_p];
                let chstring = ch.to_string();
                if true ==  (is_string_numeric(chstring)){
                    beforeplaces = beforeplaces + 1;
                }
                else if true ==  (ch.to_string() == "."){
                    beforeplaces = beforeplaces + 1;
                }
                else if true ==  (ch.to_string() == "-"){
                    beforeplaces = beforeplaces + 1;
                }
                else{
                    break;
                }
            }
            loop{
                ch_p = (int_cop  + afterplaces + 1) as usize;
                if ch_p == new_input.len(){
                    break;
                }
                let ch = &new_input[ch_p..ch_p+1];
                let chstring = ch.to_string();
                if true == (is_string_numeric(chstring)){
                    afterplaces = afterplaces + 1;
                }
                else if true == (ch.to_string() == "."){
                    afterplaces = afterplaces + 1;
                }
                else if true == (ch.to_string() == "-"){
                    afterplaces = afterplaces + 1;
                }
                else{
                    break;
                }
                println!("Afterplaces iterated");
            }
            println!("{}", beforeplaces);
            let firstnum = &new_input[(int_cop-beforeplaces) as usize..(int_cop) as usize];
            let firstnumf32:f32 = firstnum.parse::<f32>().unwrap();
            println!("2: firstnum is {}",firstnum);
            let mut secnum;
            secnum =&new_input[(int_cop+1) as usize..(int_cop+afterplaces+1) as usize];
            println!("3: secnum is {}",secnum);
            let secnumf32:f32 = secnum.parse::<f32>().unwrap();
            if current_operator == exponent{
                answer = firstnumf32.powf(secnumf32);
            }
            if current_operator ==  multiplication{
                answer = firstnumf32 * secnumf32;
            }
            if current_operator == division{
                answer = firstnumf32 / secnumf32;
            }
            if current_operator == addition{
                answer = firstnumf32 + secnumf32;
            }
            if current_operator == subtraction{
                answer = firstnumf32 - secnumf32;
            }
            println!("The answer is {}", answer);
            let mut coolstring = new_input;
            println!("4: beforeplaces is {}\n5: afterplaces is {}",beforeplaces,afterplaces);
            let inputcash1 = &coolstring[0 as usize..int_cop as usize-beforeplaces as usize];
            let inputcash2 = answer.to_string();
            let inputcash3 = &coolstring[(int_cop +afterplaces+1) as usize .. coolstring.len() as usize];
            new_input = ("").parse().unwrap();
            new_input.push_str(inputcash1);
            new_input.push_str(&inputcash2);
            new_input.push_str(inputcash3);

        }
        let mut new_input = (&input[(most_inner_parth+1) as usize..(most_inner_parth +places_after_most_inner) as usize]).to_string();

        println!("The answer is {}", answer);
        let mut inputclone = input.clone();
        let inputcash1 = &inputclone[0 as usize..most_inner_parth as usize];
        let inputcash2 = answer.to_string();
        let inputcash3 = &inputclone[(most_inner_parth + places_after_most_inner +1) as usize..(input.len()) as usize];
        input = ("").parse().unwrap();
        input.push_str(inputcash1);
        input.push_str(&inputcash2);
        input.push_str(inputcash3);
        num_of_parth = num_of_parth -1;
        operator_places.clear();
    }
    return answer;
}
fn is_string_numeric(str: String) -> bool {
    let mut numeric = 0;
    for c in str.chars() {
        if !c.is_numeric() {
            numeric = 1;
        }
    }
    if numeric == 1{
        return false;
    }else{
        return true;
    }
}
