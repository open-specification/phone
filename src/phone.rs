pub fn get_country(country_code:&str) -> &str {

    if country_code == "1" { return "United States of America, Canada"; }
    if country_code == "7" { return "Kazakhstan, Russia"; }
    if country_code == "20" { return "Egypt"; }
    if country_code == "27" { return "South Africa"; }
    if country_code == "30" { return "Greece"; }
    if country_code == "31" { return "Netherlands"; }
    if country_code == "32" { return "Belgium"; }
    if country_code == "33" { return "France"; }
    if country_code == "34" { return "Spain"; }
    if country_code == "36" { return "Hungary"; }
    if country_code == "39" { return "Italy"; }
    if country_code == "40" { return "Romania"; }
    if country_code == "41" { return "Switzerland"; }
    if country_code == "43" { return "Austria"; }
    if country_code == "44" { return "United Kingdom"; }
    if country_code == "45" { return "Denmark"; }
    if country_code == "46" { return "Sweden"; }
    if country_code == "47" { return "Norway, Svalbard and Jan Mayen"; }
    if country_code == "48" { return "Poland"; }
    if country_code == "49" { return "Germany"; }
    if country_code == "51" { return "Peru"; }
    if country_code == "52" { return "Mexico"; }
    if country_code == "53" { return "Cuba"; }
    if country_code == "54" { return "Argentina"; }
    if country_code == "55" { return "Brazil"; }
    if country_code == "56" { return "Chile"; }
    if country_code == "57" { return "Colombia"; }
    if country_code == "58" { return "Venezuela"; }
    if country_code == "60" { return "Malaysia"; }
    if country_code == "61" { return "Australia, Christmas Island, Cocos Islands"; }
    if country_code == "62" { return "Indonesia"; }
    if country_code == "63" { return "Philippines"; }
    if country_code == "64" { return "New Zealand, Pitcairn"; }
    if country_code == "65" { return "Singapore"; }
    if country_code == "66" { return "Thailand"; }
    if country_code == "81" { return "Japan"; }
    if country_code == "82" { return "South Korea"; }
    if country_code == "84" { return "Vietnam"; }
    if country_code == "86" { return "China"; }
    if country_code == "90" { return "Turkey"; }
    if country_code == "91" { return "India"; }
    if country_code == "92" { return "Pakistan"; }
    if country_code == "93" { return "Afghanistan"; }
    if country_code == "94" { return "Sri Lanka"; }
    if country_code == "95" { return "Myanmar"; }
    if country_code == "98" { return "Iran"; }

    // STOPPED AT DOUBLE DIGITS, ADD THE REST USING THIS WEBSITE
    // https://countrycode.org/

    return "";

}

pub struct PhoneInfo {

    pub number_type:String,
    pub area_code:String,

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