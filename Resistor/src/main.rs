extern crate core;

use std::fmt::{Error, format};
use std::fs::File;
use std::io::Read;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use clearscreen;

use std::{
    io::{stdin,stdout,Write},
    thread,
};
use std::process::exit;

use colored::Colorize;

enum ColorCode{
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Gray,
    White,
    Gold,
    Silver,
}
impl ColorCode{
    fn value(&self) ->Result<i64, String>{
        match *self{
            ColorCode::Black => Ok(0),
            ColorCode::Brown => Ok(1),
            ColorCode::Red => Ok(2),
            ColorCode::Orange => Ok(3),
            ColorCode::Yellow => Ok(4),
            ColorCode::Green => Ok(5),
            ColorCode::Blue => Ok(6),
            ColorCode::Violet => Ok(7),
            ColorCode::Gray => Ok(8),
            ColorCode::White => Ok(9),
            ColorCode::Gold => Ok(10),
            ColorCode::Silver => Ok(11),
        }
    }
    fn get_color_code(n:i32)->Result<ColorCode, String>{
        match n {
            0 => Ok(ColorCode::Black),
            1 => Ok(ColorCode::Brown),
            2 => Ok(ColorCode::Red),
            3 => Ok(ColorCode::Orange),
            4 => Ok(ColorCode::Yellow),
            5 => Ok(ColorCode::Green),
            6 => Ok(ColorCode::Blue),
            7 => Ok(ColorCode::Violet),
            8 => Ok(ColorCode::Gray),
            9 => Ok(ColorCode::White),
           10 => Ok(ColorCode::Gold),
           11 => Ok(ColorCode::Silver),
            _ => Err("Wrong input for colour code".to_string()),
        }
    }
    fn get_color_tf(&self) -> Result<f32, String>{
        match self{
            ColorCode::Brown => Ok(1.0),
            ColorCode::Red => Ok(2.0),
            ColorCode::Orange => Ok(3.0),
            ColorCode::Yellow => Ok(4.0),
            ColorCode::Green => Ok(0.50),
            ColorCode::Blue => Ok(0.25),
            ColorCode::Violet => Ok(0.10),
            ColorCode::Gray => Ok(0.05),
            ColorCode::Gold => Ok(5.0),
            ColorCode::Silver => Ok(10.0),
            _ => Err("Tolerance facotr for this color not available".to_string()),
        }
    }
}

struct Resitor{
    pub no_of_bands:u8,
    pub band_colors:Vec<ColorCode>,
}

impl Resitor{
    fn calculate(self)->String{
        let mut value:i64 = 0;
        let bands_count = self.no_of_bands;
        value = (&self.band_colors[0].value().unwrap()*10 + &self.band_colors[1].value().unwrap()*1);

        match bands_count {
           3 | 4 => value = value * 10_i64.pow(self.band_colors[2].value().unwrap() as u32),
           5 => value = ((&self.band_colors[0].value().unwrap() * 100) + (&self.band_colors[1].value().unwrap() * 10) + (&self.band_colors[2].value().unwrap() * 1)) * (10_i64.pow(self.band_colors[3].value().unwrap() as u32)),
           _ => {
               println!(" for these bands, no calculation available");
               exit(1);
           }
        }

        let mut resistance_value = String::from("");
        if value > 999 && value <= 999999 {
            let v:f32 = (value as f32 / 1000.0) as f32;
            resistance_value.push_str(&*v.to_string());
            resistance_value.push_str("KΩ");
        }else if value > 999999{
            let v:f32 = (value as f32 / 1000000.0) as f32;
            resistance_value.push_str(&*v.to_string());
            resistance_value.push_str("MΩ");
        }else{
            resistance_value.push_str(&*value.to_string());
            resistance_value.push_str("Ω");
        }

        match bands_count{
            3 =>resistance_value.push_str(" , tolerance factor = +-20%"),
            4 | 5 =>{
                let tf = self.band_colors.last().unwrap().get_color_tf().unwrap();
                resistance_value.push_str(" , tolerance = +-");
                resistance_value.push_str(&*tf.to_string());
                resistance_value.push_str("%");
            }
            _ => {}
        }
        resistance_value
    }
}

fn screen_input()->Resitor{
    let mut bands_vec:Vec<ColorCode> = Vec::new();

    let mut bands=String::new();
    print!(" Enter how many bands 3, 4 or 5?: ");
    let _=stdout().flush();
    stdin().read_line(&mut bands).expect(" Did not enter a correct string");
    let mut no_of_bands = 0;
    match bands.trim().parse::<u8>(){
        Ok(v) => {
            no_of_bands = v;
            if !(no_of_bands >=3 && no_of_bands <= 5){
                println!(" wrong input of the bands");
                exit(1);
            }
        }
        Err(_) => {
                println!(" wrong input, a numeric value expected");
                exit(1);
        }
    }

    let mut band1=String::new();
    print!(" Color code of 1st Band?: ");
    let _=stdout().flush();
    stdin().read_line(&mut band1).expect(" Did not enter a correct string");
    let band1 = validate_color_code(band1.trim());
    bands_vec.push(ColorCode::get_color_code(band1).unwrap());


    let mut band2=String::new();
    print!(" Color code of 2nd Band?: ");
    let _=stdout().flush();
    stdin().read_line(&mut band2).expect(" Did not enter a correct string");
    let band2 = validate_color_code(band2.trim());
    bands_vec.push(ColorCode::get_color_code(band2).unwrap());

    let mut band3=String::new();
    print!(" Color code of 3rd Band?: ");
    let _=stdout().flush();
    stdin().read_line(&mut band3).expect(" Did not enter a correct string");
    let band3 = validate_color_code(band3.trim());
    bands_vec.push(ColorCode::get_color_code(band3).unwrap());

    let mut band4;
    let mut band5;

    if no_of_bands == 4{
        band4 = String::new();
        print!(" Color code of 4th Band?: ");
        let _=stdout().flush();
        stdin().read_line(&mut band4).expect(" Did not enter a correct string");
        let band4 = validate_color_code(band4.trim());
        bands_vec.push(ColorCode::get_color_code(band4).unwrap());
    }
    if no_of_bands == 5{
        band4=String::new();
        print!(" Color code of 4th Band?: ");
        let _=stdout().flush();
        stdin().read_line(&mut band4).expect(" Did not enter a correct string");
        let band4 = validate_color_code(band4.trim());
        bands_vec.push(ColorCode::get_color_code(band4).unwrap());

        band5=String::new();
        print!(" Color code of 5th Band?: ");
        let _=stdout().flush();
        stdin().read_line(&mut band5).expect(" Did not enter a correct string");
        let band5 = validate_color_code(band5.trim());
        bands_vec.push(ColorCode::get_color_code(band5).unwrap());
    }
    println!("");
    print!(" -▪");
    for color in bands_vec.iter(){
        match color {
            ColorCode::Black =>print!("{}","█".truecolor(0,0,0)),
            ColorCode::Brown =>print!("{}","█".truecolor(150,75,0)),
            ColorCode::Red =>print!("{}","█".truecolor(255,0,0)),
            ColorCode::Orange =>print!("{}","█".truecolor(255,165,0)),
            ColorCode::Yellow =>print!("{}","█".truecolor(255,255,0)),
            ColorCode::Green =>print!("{}","█".truecolor(0,255,0)),
            ColorCode::Blue =>print!("{}","█".truecolor(0,0,255)),
            ColorCode::Violet =>print!("{}","█".truecolor(155,38,182)),
            ColorCode::Gray =>print!("{}","█".truecolor(128,128,128)),
            ColorCode::White =>print!("{}","█".truecolor(255,255,255)),
            ColorCode::Gold =>print!("{}","█".truecolor(255,215,0)),
            ColorCode::Silver =>print!("{}","█".truecolor(192,192,192)),
        }
    }
    print!("▪-");
    println!("");

    Resitor{
        no_of_bands,
        band_colors: bands_vec,
    }
}

fn validate_color_code(input_color_code:&str)->i32{
    let mut color_code;
    match input_color_code.parse::<i32>(){
        Ok(v) => color_code = v,
        Err(_) => {
                    println!(" wrong input, a numeric value expected");
                    exit(1);
                   }
    }
    match ColorCode::get_color_code(color_code){
        Ok(_)=>{}
        Err(e)=>{
            println!("{:?}", e);
            exit(1);
        }
    }
    color_code
}

fn display_color_codes(){
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.apply_modifier(UTF8_SOLID_INNER_BORDERS);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let black_code = Cell::new("0").set_alignment(CellAlignment::Center);
    let cell_black = Cell::new("Black").bg(Color::Black);

    let brown_code = Cell::new("1").set_alignment(CellAlignment::Center);
    let cell_brown = Cell::new("Brown").bg(Color::Rgb{ r: 150, g: 75, b: 0 });

    let red_code = Cell::new("2").set_alignment(CellAlignment::Center);
    let cell_red = Cell::new("Red").bg(Color::Rgb{ r: 255, g: 0, b: 0 }).fg(Color::Black);

    let orange_code = Cell::new("3").set_alignment(CellAlignment::Center);
    let cell_orange = Cell::new("Orange").bg(Color::Rgb{ r: 255, g: 165, b: 0 }).fg(Color::Black);

    let yellow_code = Cell::new("4").set_alignment(CellAlignment::Center);
    let cell_yellow = Cell::new("Yellow").bg(Color::Rgb{ r: 255, g: 255, b: 0 }).fg(Color::Black);

    let green_code = Cell::new("5").set_alignment(CellAlignment::Center);
    let cell_green = Cell::new("Green").bg(Color::Rgb{ r: 0, g: 255, b: 0 }).fg(Color::Black);

    let blue_code = Cell::new("6").set_alignment(CellAlignment::Center);
    let cell_blue = Cell::new("Blue").bg(Color::Rgb{ r: 0, g: 0, b: 255 }).fg(Color::Black);

    let violet_code = Cell::new("7").set_alignment(CellAlignment::Center);
    let cell_viloet = Cell::new("Viloet").bg(Color::Rgb{ r: 127, g: 0, b: 255 });

    let gray_code = Cell::new("8").set_alignment(CellAlignment::Center);
    let cell_gray = Cell::new("Gray").bg(Color::Rgb{ r: 128, g: 128, b: 128 });

    let white_code = Cell::new("9").set_alignment(CellAlignment::Center);
    let cell_white = Cell::new("White").bg(Color::White).fg(Color::Black);

    let gold_code = Cell::new("10").set_alignment(CellAlignment::Center);
    let cell_gold = Cell::new("Gold").bg(Color::Rgb{ r: 255, g: 215, b: 0 }).fg(Color::Black);

    let silver_code = Cell::new("11").set_alignment(CellAlignment::Center);
    let cell_silver = Cell::new("Silver").bg(Color::Rgb{ r: 192, g: 192, b: 192 }).fg(Color::Black);

    table.add_row(
                    vec![black_code,brown_code,red_code,orange_code,yellow_code,
                            green_code,blue_code,violet_code,gray_code,white_code,gold_code,silver_code]
    );

    table.add_row(
                    vec![cell_black,cell_brown,cell_red,cell_orange,cell_yellow,cell_green,
                            cell_blue,cell_viloet,cell_gray,cell_white,cell_gold,cell_silver]
    );

    println!("");
    println!("{}",table);
    println!("");
    println!("  3 Band Resistor    4 Band Resistor     5 Band Resistor");
    println!("  __█▀█▀█▀█__        __█▀█▀█▀███▀█__     __█▀█▀█▀█▀███▀█__");
    println!("    █▄█▄█▄█            █▄█▄█▄███▄█         █▄█▄█▄█▄███▄█  ");
    println!("");
}


fn main() {
    loop{
        clearscreen::clear().expect("failed to clear screen");
        display_color_codes();
        let resistor = screen_input();

        println!("");
        println!(" Resistance value = {:?}", resistor.calculate());

        let mut choice = String::new();
        println!("");
        print!(" press any key to continue or 'x' to exit : ");
        let _=stdout().flush();
        stdin().read_line(&mut choice).expect("Did not enter a correct string");
        if "x" == choice.trim() {
            break;
        }
    }
}