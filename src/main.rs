fn main(){
    use std::io;
    let mut input = String::new();
    println!("input equation");
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("input received")
        },
        Err(e) => println!("input failed. Please report to github.")
    }

    graph(input);
}

fn graph(input: String){
    extern crate kiss3d;
    extern crate nalgebra as na;
    solve_string( input);
    use kiss3d::light::Light;
    use kiss3d::window::Window;
    use na::Point3;
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
        //makes refrence
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
    let mut operator_places = vec![];
    let exponent = "^";
    let multiplication = "*";
    let division = "/";
    let addition = "+";
    let subtraction = "#";
    // code here
    for i in 1..input.len(){
        let mut current_letter = &input[i..i+1];
        if current_letter == exponent{
            operator_places.push(exponent);
        }
        else if current_letter == multiplication{
            operator_places.push(multiplication);
        }

        else if current_letter == division{
            operator_places.push(division);
        }

        else if current_letter == addition{
            operator_places.push(addition);
        }

        else if current_letter == subtraction{
            operator_places.push(subtraction);
        }
    }
    for i in 0..opperator_places.len() {
        let beforeplaces = 0;
        let afterplaces = 1;
        let current_operator = opperator_places[i];
        //let current_operator_pos = (input.index(current_operator));
    }
    //return exampleoutput1;
}