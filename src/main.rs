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


fn main(){
        kiss_ui::show_gui(|| {
            Dialog::new(
                Vertical::new(
                    children![
                    Label::new("Enter a message:"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("equ_raw"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("min"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("max"),
                    TextBox::new()
                        .set_visible_columns(20)
                        .set_name("rate"),
                    Button::new()
                        .set_label("Graph")
                        .set_onclick(show_alert_message),
                ]
                )
            )
                .set_title("Hello, world!")
                .set_size_pixels(640, 480)
        });
    use std::io;
    let mut input = String::new();
    println!("input equation");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("input received")
        },
        Err(e) => println!("input failed. Please report to github.")
    }
}

fn show_alert_message(clicked: Button) {
    println!("I have awoken?");
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
    graph(equ.to_string(), min.to_string(), max.to_string(), rate.to_string());
}

fn graph(equation: String, min: String, max: String, rate: String){
    solve_string(equation);
    let mut corda;
    let mut cordb;
    let boxx = [0.0, 0.0, 0.0, 0.0];
    let boxy = [10.0, 0.0, -10.0,0.0];
    let boxz = [0.0, 10.0, 0.0,-10.0];
    let examplex = [0.0, 0.0, 0.0, 0.0];
    let exampley = [10.0, 0.0, -10.0,0.0];
    let examplez = [0.0, 10.0, 0.0,-10.0];
    let mut window = Window::new("Kiss3d: lines");
    window.set_light(Light::StickToCamera);
    while window.render() {
        //makes reference
        for i in  1..boxx.len(){
            let bx = boxx[i];
            let by = boxy[i];
            let bz = boxz[i];
            corda = Point3::new(0.0, 0.0, 0.0);
            cordb = Point3::new(bx, by, bz);
            window.draw_line(&corda, &cordb, &Point3::new(1.0, 0.0, 0.0));
        }
        //makes graph
        for i in  1..examplex.len(){
            let ax = examplex[i-1];
            let ay = exampley[i-1];
            let az = examplez[i-1];
            let bx = examplex[i];
            let by = exampley[i];
            let bz = examplez[i];
            corda = Point3::new(ax, ay, az);
            cordb = Point3::new(bx, by, bz);
            window.draw_line(&corda, &cordb, &Point3::new(1.0, 0.0, 0.0));
        }
    }
}
fn solve_string(input :String){
    println!("ran solve string");
    let mut operator_places = vec![];
    let exponent = "^";
    let multiplication = "*";
    let division = "/";
    let addition = "+";
    let subtraction = "#";
    // code here
    for i in 0..input.len() {
        let mut current_letter = &input[i..i + 1];
        if current_letter == exponent {
            operator_places.push(exponent);
        }
    }
    for i in 0..input.len() {
        let mut current_letter = &input[i..i + 1];
        if current_letter == multiplication {
            operator_places.push(multiplication);
        } else if current_letter == division {
            operator_places.push(division);
        }
    }
    for i in 0..input.len() {
        let mut current_letter = &input[i..i + 1];
        if current_letter == addition{
            operator_places.push(addition);
        }

        else if current_letter == subtraction{
            operator_places.push(subtraction);
        }
    }
    for i in 0..operator_places.len() {
        println!("ran operator_places");
        let mut beforeplaces = 0;
        let mut afterplaces = 1;
        let current_operator = operator_places[i];
        let mut int_cop:i32 = 0;
        for s in 0..input.len(){
            let mut current_letter = &input[s..s + 1];
            if current_operator == current_letter{
                let str_cop = s.to_string();
                int_cop = str_cop.parse::<i32>().unwrap();
            }
        }

        println!("hi {:?}", int_cop);

        let isletter:bool = true;
        let mut ch_p:usize;
        loop{
            ch_p = (int_cop - beforeplaces) as usize;
            if ch_p == 0{
                break;
            }
            println!("ch_p is {}", ch_p);
            let ch = &input[ch_p-1..ch_p];
            let chstring = ch.to_string();
            println!("chstring is{}", chstring);
            if isletter ==  (is_string_numeric(chstring)){
                println!("added beforeplace is nubmer");
                beforeplaces = beforeplaces + 1;
            }
            else if isletter ==  (ch.to_string() == "."){
                println!("added beforeplace is .");
                beforeplaces = beforeplaces + 1;
            }
            else{
                break;
            }
        }
        println!("beforeplaces {}", beforeplaces);
        loop{
            ch_p = (int_cop  + afterplaces) as usize;
            if ch_p == input.len(){
                break;
            }
            println!("ch_p is {}", ch_p);
            let ch = &input[ch_p-1..ch_p];
            let chstring = ch.to_string();
            println!("chstring is{}", chstring);
            if isletter == (is_string_numeric(chstring)){
                println!("added beforeplace is nubmer");
                afterplaces = afterplaces + 1;
            }
            else if isletter == (ch.to_string() == "."){
                println!("added beforeplace is .");
                afterplaces = afterplaces + 1;
            }
            else{
                break;
            }
        }
        println!("after places{}", afterplaces);
    }
    //return exampleoutput1;
}
fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}