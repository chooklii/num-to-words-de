use regex::Regex;

const BASE_NUMBERS: [&str; 20] = [
    "", "ein", "zwei", "drei", "vier", "fünf", "sechs", "sieben", "acht", "neun", "zehn", "elf",
    "zwölf","dreizehn", "vierzehn", "fünfzehn", "sechszehn", "siebzehn", "achtzehn", "neunzehn"
];
const TENS: [&str; 10] = [
    "", "zehn", "zwanzig", "dreißig", "vierzig", "fünfzig", "sechzig", "siebzig", "achtzig",
    "neunzig",
];
const ONE: &str = "eins";
const HUNDRET: &str = "hundert";
const THOUSEND: &str = "tausend";
const MILLION_SINGLE: &str = "million";
const MILLION: &str = "millionen";
const BILLION_SINGLE: &str = "milliarde";
const BILLION: &str = "milliarden";
const AND: &str = "und";

pub struct NumConstruct {
    hundret: u8,
    tens: u8,
    height: Height,
}

pub enum Height {
    MRD,
    MIL,
    T,
    NONE
}

fn base_numbers(input_number: u8, height: &Height) -> String{
    if input_number == 1 && matches!(height,Height::NONE){
        return String::from(ONE);
    }
    String::from(BASE_NUMBERS[input_number as usize])
}

fn split_double_digit(input_number: u8) -> [u8; 2] {
    let re = Regex::new(r"(\d{1})(\d{1})").unwrap();
    let regex_input = input_number.to_string();
    let captures = re.captures(&regex_input).unwrap();
    [
        captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
    ]
}

fn thirteen_to_hundret_to_string(input_number: u8) -> String {
    let mut result_value = String::from("");
    let split = split_double_digit(input_number);

    let singles = split[0];
    let doubles = split[1];
    if singles != 0{
        result_value.push_str(BASE_NUMBERS[singles as usize]);
        if doubles != 0{
            result_value.push_str(AND);
        }
    }
    if doubles != 0{
        result_value.push_str(TENS[doubles as usize])
    }
    
    result_value
}

pub fn sub_hundret_to_string(input_number: u8, height: &Height) -> String {
    match input_number {
        1..=19 => base_numbers(input_number, &height),
        20..100 => thirteen_to_hundret_to_string(input_number),
        _ => String::from(""),
    }
}

pub fn split(input_number: u32) -> Vec<NumConstruct> {
    // Add dummy Zeros to regex into correct fields from the back
    let mut reg_number = String::from("0000000000");
    reg_number.push_str(&input_number.to_string());
    let re = Regex::new(r"(\d{1})(\d{1})(\d{2})(\d{1})(\d{2})(\d{1})(\d{2})$").unwrap();

    let captures = re.captures(&reg_number).unwrap();

    let mut splits = Vec::new();
    splits.push(NumConstruct {
        hundret: 0,
        tens: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
        height: Height::MRD,
    });

    splits.push(NumConstruct {
        hundret: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        tens: captures.get(3).unwrap().as_str().parse::<u8>().unwrap(),
        height: Height::MIL,
    });

    splits.push(NumConstruct {
        hundret: captures.get(4).unwrap().as_str().parse::<u8>().unwrap(),
        tens: captures.get(5).unwrap().as_str().parse::<u8>().unwrap(),
        height: Height::T,
    });

    splits.push(NumConstruct {
        hundret: captures.get(6).unwrap().as_str().parse::<u8>().unwrap(),
        tens: captures.get(7).unwrap().as_str().parse::<u8>().unwrap(),
        height: Height::NONE,
    });

    splits
}

fn get_height_value(height: &Height, single: bool) -> &'static str {
    match height {
        Height::MRD => {
            if single {
                BILLION_SINGLE
            } else {
                BILLION
            }
        }
        Height::MIL => {
            if single {
                MILLION_SINGLE
            } else {
                MILLION
            }
        }
        Height::T => THOUSEND,
        Height::NONE => ""
    }
}

fn get_value(number: &NumConstruct) -> String {
    let mut return_string = String::new();

    if number.hundret == 0 && number.tens == 0 {
        return return_string;
    }

    if number.hundret != 0 {
        return_string.push_str(BASE_NUMBERS[number.hundret as usize]);
        return_string.push_str(HUNDRET);
    }
    return_string.push_str(&sub_hundret_to_string(number.tens, &number.height));
    let single = number.hundret == 0 && number.tens == 1;
    return_string.push_str(get_height_value(&number.height, single));
    return_string
}

fn main() {
    let mut returnValue = String::new();
    let number = 1001;
    let number_split_to_party = split(number);

    for i in number_split_to_party.iter() {
        returnValue.push_str(get_value(i).as_str());
    }

    println!("Value {returnValue}");
}
