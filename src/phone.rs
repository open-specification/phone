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