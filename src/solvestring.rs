pub mod solvestring{

pub fn solve_string(mut input: String) -> f32 {
    //declares stuff.
    let mut answer: f32 = 404.0;
    let mut operator_places = vec![];
    let exponent = "^";
    let multiplication = "*";
    let division = "/";
    let addition = "+";
    let subtraction = "~";
    let mut current_letter;
    let mut num_of_parth = 0;
    let mut most_inner_parth;
    let mut places_after_most_inner;
    let mut runner;
    //creates a clone of input in order to retain the original input.
    let before_input = input.clone();
    //adds parenthesis on either side of the equation.
    input = "".to_string();
    input.push_str("(");
    input.push_str(&before_input);
    input.push_str(")");
    //finds the number of opening parenthesis
    for i in 0..input.len() {
        current_letter = &input[i..i + 1];
        if current_letter == "(" {
            num_of_parth = num_of_parth + 1
        }
    }
    //runs the solving part of solve string once for each opening parenthesis
    while num_of_parth > 0 {
        //resets variables
        most_inner_parth = 0;
        places_after_most_inner = 0;
        runner = 0;
        //determines the location of the innermost opening and closing parenthesis
        for i in 0..input.len() {
            current_letter = &input[i..i + 1];
            if current_letter == "(" {
                most_inner_parth = most_inner_parth + runner;
                runner = 0;
            } else if current_letter == ")" {
                break;
            }
            runner = runner + 1;
        }
        for i in most_inner_parth..input.len() {
            current_letter = &input[i..i + 1];
            if current_letter == ")" {
                break;
            } else {
                places_after_most_inner = places_after_most_inner + 1;
            }
        }
        // creates an input to be fed to the operator detector and solver.
        let mut new_input = (&input[(most_inner_parth + 1) as usize..(most_inner_parth + places_after_most_inner) as usize]).to_string();
        //adds operations in the order of pemdas
        for i in 0..new_input.len() {
            current_letter = &new_input[i..i + 1];
            if current_letter == exponent {
                operator_places.push(exponent);
            }
        }
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
            if current_letter == addition {
                operator_places.push(addition);
            } else if current_letter == subtraction {
                operator_places.push(subtraction);
            }
        }
        //finds the place of the current operator.
        for i in 0..operator_places.len() {
            let mut beforeplaces = 1;
            let mut afterplaces = 1;
            let current_operator = operator_places[i];
            let mut int_cop: i32 = 0;
            for s in 0..new_input.len() {
                current_letter = &new_input[s..s + 1];
                if current_operator == current_letter {
                    int_cop = s as i32;
                }
            }
            let mut ch_p: usize;
            //determines the number of places before the operator that are numbers
            loop {
                ch_p = (int_cop - beforeplaces) as usize;
                if ch_p == 0 {
                    break;
                }
                let ch = (&new_input[ch_p - 1..ch_p]).to_string();
                if true == is_string_numeric(ch.clone()) || (ch == ".") || (ch == "-") {
                    beforeplaces = beforeplaces + 1;
                } else {
                    break;
                }
            }
            //determines the number of places after the operator that are numbers.
            loop {
                ch_p = (int_cop + afterplaces + 1) as usize;
                if ch_p == new_input.len() {
                    break;
                }
                let ch = (&new_input[ch_p..ch_p + 1]).to_string();
                if true == is_string_numeric(ch.clone()) || (ch == ".") || (ch == "-") {
                    afterplaces = afterplaces + 1;
                } else {
                    break;
                }
            }
            //takes new input and before places and comes up with first number to be operatored on.
            let firstnumf32 = (&new_input[(int_cop - beforeplaces) as usize..(int_cop) as usize]).parse::<f32>().unwrap();
            //does the same as first num except after places.
            let secnumf32 = (&new_input[(int_cop + 1) as usize..(int_cop + afterplaces + 1) as usize]).parse::<f32>().unwrap();
            //does the corresponding math as requested by current operator.
            if current_operator == exponent {
                answer = firstnumf32.powf(secnumf32);
            }
            if current_operator == multiplication {
                answer = firstnumf32 * secnumf32;
            }
            if current_operator == division {
                answer = firstnumf32 / secnumf32;
            }
            if current_operator == addition {
                answer = firstnumf32 + secnumf32;
            }
            if current_operator == subtraction {
                answer = firstnumf32 - secnumf32;
            }
            println!("{}", answer);
            //edits new input to equal new input minus the operator and numbers acted upon.
            let inputcash1 = &new_input.clone()[0 as usize..int_cop as usize - beforeplaces as usize];
            let inputcash2 = &answer.to_string() as &str;
            let inputcash3 = &new_input.clone()[(int_cop + afterplaces + 1) as usize..new_input.len() as usize];
            new_input = ("").parse().unwrap();
            new_input.push_str(inputcash1);
            new_input.push_str(inputcash2);
            new_input.push_str(inputcash3);
        }
        //lets input be modified from the answer of the parenthesis
        let inputclone = input.clone();
        let inputcash1 = &inputclone[0 as usize..most_inner_parth as usize];
        let inputcash2 = &answer.to_string() as &str;
        let inputcash3 = &inputclone[(most_inner_parth + places_after_most_inner + 1) as usize..(input.len()) as usize];
        input = ("").parse().unwrap();
        input.push_str(inputcash1);
        input.push_str(inputcash2);
        input.push_str(inputcash3);
        num_of_parth = num_of_parth - 1;
        operator_places.clear();
    }
    //returns the answer as f32
    return answer;
}

//determines if the characters in a string are numeric. written by me
fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
}