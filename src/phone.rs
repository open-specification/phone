macro_rules! get_country_match {

    ($country_code:ident, ($(($country:expr, $number:expr)), *)) => {

        match $country_code {

            $( stringify!($number) => $country, )*
            _ => "",

        }

    }

}

pub fn get_country(country_code:&str) -> &str {

    // return get_country_match!(country_code, ((include_str!("./country_codes.txt"), 1)));

    return match country_code {

        "1" => "United States of America, Canada",
        "7" => "Kazakhstan, Russia",
        "20" => "Egypt",
        "27" => "South Africa",
        "30" => "Greece",
        "31" => "Netherlands",
        "32" => "Belgium",
        "33" => "France",
        "34" => "Spain",
        "36" => "Hungary",
        "39" => "Italy",
        "40" => "Romania",
        "41" => "Switzerland",
        "43" => "Austria",
        "44" => "United Kingdom",
        "45" => "Denmark",
        "46" => "Sweden",
        "47" => "Norway, Svalbard and Jan Mayen",
        "48" => "Poland",
        "49" => "Germany",
        "51" => "Peru",
        "52" => "Mexico",
        "53" => "Cuba",
        "54" => "Argentina",
        "55" => "Brazil",
        "56" => "Chile",
        "57" => "Colombia",
        "58" => "Venezuela",
        "60" => "Malaysia",
        "61" => "Australia, Christmas Island, Cocos Islands",
        "62" => "Indonesia",
        "63" => "Philippines",
        "64" => "New Zealand, Pitcairn",
        "65" => "Singapore",
        "66" => "Thailand",
        "81" => "Japan",
        "82" => "South Korea",
        "84" => "Vietnam",
        "86" => "China",
        "90" => "Turkey",
        "91" => "India",
        "92" => "Pakistan",
        "93" => "Afghanistan",
        "94" => "Sri Lanka",
        "95" => "Myanmar",
        "98" => "Iran",
        _ => "",

    }

    // STOPPED AT DOUBLE DIGITS, ADD THE REST USING THIS WEBSITE
    // https://countrycode.org/

    // return "";

}

pub struct PhoneInfo {

    pub number_type:String,
    pub area_code:String,
    // pub valid:bool;

}

// Uses the "North American Numbering Plan" system to get information on phone numbers.
pub fn get_nanp_info(phone_number:&str) -> PhoneInfo {

    if phone_number.len() < 3 { return PhoneInfo { number_type: "null".to_string(), area_code: phone_number.to_string() }; }

    let area_code = &phone_number[0..3];
    let area_code_number:u16 = area_code.parse().unwrap();
    let mut number_type:&str = "null";

    if area_code_number < 200 { number_type = "system"; }

    let mut x = 200;
    loop {

        if x > 900 { break; }

        if area_code_number >= x && area_code_number < x + 11 { number_type = "area"; break; }
        if area_code_number == x + 11 { number_type = "service"; break; }
        if area_code_number >= x + 12 && area_code_number < x + 20 { number_type = "area"; break; }
        if area_code_number >= x + 20 && area_code_number < x + 100 { number_type = "area"; break; }

        x = x + 100;

    }

    // Check if Valid Phone Number

    return PhoneInfo { number_type: number_type.to_string(), area_code: area_code.to_string() };
    

}

pub fn get_general_info(country_name:&str, phone_number:&str) -> PhoneInfo {

    let country_code = get_country_code(country_name);

    if country_code == 1 { return get_nanp_info(phone_number); }

    return PhoneInfo { number_type: "null".to_owned(), area_code: "".to_owned() };

}

pub fn get_country_code(country_name:&str) -> u32 {
 
    if country_name == "CA" || country_name == "CAN" { return 1; }
    if country_name == "US" || country_name == "USA" { return 1; }

    return 0;

}

pub fn e164_format(country_name:&str, phone_number:&str) -> String {

    let country_code = get_country_code(country_name);

    if country_code == 0 { return format!("{}", phone_number); }

    return format!("+{}{}", country_code, phone_number);

}

pub fn branched_format(country_name:&str, phone_number:&str) -> String {

    let country_code = get_country_code(country_name);
    let mut chars = phone_number.chars();

    let formatted_number = match phone_number.len() {

        0 => "".to_owned(),
        1 => format!("{}", phone_number.chars().nth(0).unwrap()),
        2 => format!("{}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap()),
        3 => format!("({}{}{})", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap()),
        4 => format!("{}{} {}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap()),
        5 => format!("({}{}{}) {}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap()),
        6 => format!("({}{}{}) {}{}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap(), phone_number.chars().nth(5).unwrap()),
        7 => format!("({}{}{}) {}{}{} {}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap(), phone_number.chars().nth(5).unwrap(), phone_number.chars().nth(6).unwrap()),
        8 => format!("{}{} {}{} {}{} {}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap(), phone_number.chars().nth(5).unwrap(), phone_number.chars().nth(6).unwrap(), phone_number.chars().nth(7).unwrap()),
        9 => format!("({}{}{}) {}{}{} {}{}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), phone_number.chars().nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap(), phone_number.chars().nth(5).unwrap(), phone_number.chars().nth(6).unwrap(), phone_number.chars().nth(7).unwrap(), phone_number.chars().nth(8).unwrap()),
        10 => format!("({}{}{}) {}{}{} {}{}{}{}", phone_number.chars().nth(0).unwrap(), phone_number.chars().nth(1).unwrap(), chars.nth(2).unwrap(), phone_number.chars().nth(3).unwrap(), phone_number.chars().nth(4).unwrap(), phone_number.chars().nth(5).unwrap(), phone_number.chars().nth(6).unwrap(), phone_number.chars().nth(7).unwrap(), phone_number.chars().nth(8).unwrap(), phone_number.chars().nth(9).unwrap()),
        _ => "".to_owned()

    };

    if country_code == 0 { return formatted_number; }

    return format!("+{} {}", country_code, formatted_number);

}