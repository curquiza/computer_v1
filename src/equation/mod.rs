mod test;
use std::collections::BTreeMap;
use crate::token;
use crate::maths;
use crate::error;

/*
** PARSING ********************************************************************
*/

fn add_coeff_in_table(token: &token::Token, vec_coeff: &mut Vec<i32>) {
    if token::is_plus(&token) {
        vec_coeff.push(1);
    } else if token::is_minus(&token) {
        vec_coeff.push(-1);
    }
}

fn get_coeff_table(tokens: &[token::Token]) -> Vec<i32> {
    let mut vec_coeff: Vec<i32> = Vec::new();
    vec_coeff.push(1);  // first member
    tokens.iter().for_each(|token| add_coeff_in_table(token, &mut vec_coeff));
    return vec_coeff;
}

fn get_exponent(member: &[token::Token]) -> u32 {
    if token::is_coefficient(&member[0]) {
        match member.len() {
            1 => return 0,
            _ => return member[2].exponent
        }
    }
    member[0].exponent
}

fn parse_f64(word: &str) -> f64 {
    match word.parse::<f64>() {
        Err(_) => 0.0,
        Ok(v) => v
    }
}

fn get_coeff(member: &[token::Token]) -> f64 {
    if token::is_indeterminate(&member[0]) {
        match member.len() {
            1 => return 1.0,
            _ => return parse_f64(&member[2].word)
        }
    }
    parse_f64(&member[0].word)
}

/*
** member :
**      a factor.
**      ex : X * 2 or X or 2
** components :
**      an hasmap.
**      - key : exponent
**      - value : coefficient
**      ex: X^3 * -13 -> key = 3 and value = -13
** side_coeff :
**      1 if left side of equation, -1 otherwise.
** coeff_tab :
**      coefficients for each equation factor before spliting (1 or -1).
**      ex : "X * 2 - X + 8" will give [1, -1, 1]
*/

fn parse_each_factor(member: &[token::Token], pos: usize, components: &mut BTreeMap<u32, f64>, side_coeff: i32, coeff_tab: &[i32]) {
    let final_coeff: f64 = (coeff_tab[pos] as f64) * (side_coeff as f64);
    let expo = get_exponent(member);
    match components.contains_key(&expo) {
        false => components.insert(get_exponent(member), final_coeff * get_coeff(member)),
        true => components.insert(get_exponent(member), components[&expo] + final_coeff * get_coeff(member))
    };
}

fn parse_sub_eq(tokens: &[token::Token], components: &mut BTreeMap<u32, f64>, side_coeff: i32) {
    let coeff_tab: Vec<i32> = get_coeff_table(tokens);
    let split = tokens.split(|t| token::is_separator_op(&t));
    split.enumerate().for_each(|(pos, member)| parse_each_factor(member, pos, components, side_coeff, &coeff_tab));
}

pub fn parse(tokens: &Vec<token::Token>) -> BTreeMap<u32, f64> {
    let mut components: BTreeMap<u32, f64> = BTreeMap::new();
    let mut split = tokens.split(|t| token::is_equal(&t));
    let (left, right) = (split.next().unwrap(), split.next().unwrap()); //TODO: Changer le unwrap
    parse_sub_eq(left, &mut components, 1);
    parse_sub_eq(right, &mut components, -1);
    components
}

/*
** DISPLAY ********************************************************************
*/

fn add_one_factor_to_eq(expo: u32, coeff: f64, s: &mut String) {
    if coeff == 0.0 {
        return ;
    }
    if coeff == 1.0 { // impossible to use float in pattern matching
        match expo {
            0 => s.push_str(" 1 +"),
            1 => s.push_str(" X +"),
            e => s.push_str(&format!(" X^{} +", e))
        }
    } else {
        match (coeff, expo) {
            (c, 0) => s.push_str(&format!(" {} +", c)),
            (c, 1) => s.push_str(&format!(" {} * X +", c)),
            (c, e) => s.push_str(&format!(" {} * X^{} +", c, e)),
        }
    }
}

fn get_reduced_form(components: &BTreeMap<u32, f64>) -> String {
    let mut equation: String = String::new();
    components.iter().for_each(|(expo, coeff)| add_one_factor_to_eq(*expo, *coeff, &mut equation));
    if equation.len() < 2 {
        return "0".to_string();
    }
    let end = equation.len() - 2;
    equation[1..end].to_string()
}

pub fn display_reduced_eq(components: &BTreeMap<u32, f64>) {
    println!("Reduced form: {} = 0", get_reduced_form(&components));
}

/*
** SOLVER *********************************************************************
*/

fn comp_expo(coeff: &f64) -> bool {
    if *coeff == 0.0 {
        return false
    };
    true
}

fn get_polynomial_degree(components: &BTreeMap<u32, f64>) -> u32 {
    match components.iter().max_by_key(|(_, v)| comp_expo(*v)) {
        None => 0,
        Some((expo, _)) => *expo
    }
}

pub fn solve(components: &BTreeMap<u32, f64>) -> Result<(), error::AppError> {
    let degree = get_polynomial_degree(components);
    println!("Polynomial degree: {}", degree);
    match degree {
        0...2 => println!("Solving..."),
        _ => return Err(error::too_hight_polynomial_degree())
    }
    Ok(())
}
