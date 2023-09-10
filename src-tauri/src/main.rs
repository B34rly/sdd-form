#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{format::ParseErrorKind, prelude::*};
use std::collections::HashMap;

#[tauri::command]
fn check_form_data(
    first_name: &str,
    last_name: &str,
    dob: &str,
    _home_phone: &str,
    mobile_phone: &str,
    address: &str,
) -> HashMap<String, String> {
    let mut errors: HashMap<String, String> = HashMap::new();
    if first_name == "" {
        errors.insert(
            ("first_name").to_string(),
            ("This cannot be left blank!").to_string(),
        );
    }
    if last_name == "" {
        errors.insert(
            ("last_name").to_string(),
            ("This cannot be left blank!").to_string(),
        );
    }
    if dob == "" {
        errors.insert(
            ("dob").to_string(),
            ("This cannot be left blank!").to_string(),
        );
    } else {
        match NaiveDate::parse_from_str(dob, "%d/%m/%Y") {
            Ok(_) => println!("date fine"),
            Err(date_error) => {
                let error_message = match date_error.kind() {
                ParseErrorKind::OutOfRange=> "The given field is out of the permitted range.",
                ParseErrorKind::Impossible=> "There is no possible date with the inputted data. Be sure to use dd/mm/YYYY format.",
                ParseErrorKind::NotEnough => "There is not enough data for a full date. Be sure to use dd/mm/YYYY format.",
                ParseErrorKind::Invalid => "There are invalid characters present. Be sure to use dd/mm/YYYY format.",
                ParseErrorKind::TooShort => "The inputted data is too short. Is this the complete date?",
                ParseErrorKind::TooLong => "The inputted data is too long. Did you make a typo?",
                ParseErrorKind::BadFormat => "The inputted data was incorrectly formatted. Be sure to use dd/mm/YYYY format.",
                _ => "An unknown error occured. Make sure the inputted data was in the dd/mm/YYYY format."
              };
                assert_eq!(
                    errors.insert(("dob").to_string(), error_message.to_string()),
                    None
                );
            }
        };
    }

    if _home_phone != "" {
        if _home_phone.replace(" ", "").len() < 10 {
            errors
                .entry("home_phone".to_string())
                .and_modify(|home_phone| {
                    format!(
                        "{} {}",
                        home_phone,
                        "Not enough characters for a home phone number.".to_string()
                    );
                })
                .or_insert("Not enough characters for a home phone number.".to_string());
        } else if _home_phone.replace(" ", "").len() > 10 {
            errors
                .entry("home_phone".to_string())
                .and_modify(|home_phone| {
                    format!(
                        "{} {}",
                        home_phone,
                        "Too many characters for a home phone number.".to_string()
                    );
                })
                .or_insert("Too many characters for a home phone number.".to_string());
        }
        if (_home_phone.chars().nth(0) != "0".chars().nth(0))
            || (_home_phone.chars().nth(0) == "+".chars().nth(0))
        {
            //use the australian not international format!
            errors
                .entry("home_phone".to_string())
                .and_modify(|home_phone| {
                    format!(
                        "{} {}",
                        home_phone,
                        "Use the Australian format, not the international format.".to_string()
                    );
                })
                .or_insert("Use the Australian format, not the international format.".to_string());
        }
    }

    if mobile_phone == "" {
        errors.insert(
            ("mobile_phone").to_string(),
            ("This cannot be left blank!").to_string(),
        );
    } else {
        if mobile_phone.replace(" ", "").len() < 10 {
            errors
                .entry("mobile_phone".to_string())
                .and_modify(|mobile_phone| {
                    format!(
                        "{} {}",
                        mobile_phone,
                        "Not enough characters for a mobile phone number.".to_string()
                    );
                })
                .or_insert("Not enough characters for a mobile phone number.".to_string());
        } else if mobile_phone.replace(" ", "").len() < 10 {
            errors
                .entry("mobile_phone".to_string())
                .and_modify(|mobile_phone| {
                    format!(
                        "{} {}",
                        mobile_phone,
                        "Too many characters for a mobile phone number.".to_string()
                    );
                })
                .or_insert("Too many characters for a mobile phone number.".to_string());
        }
        for char in mobile_phone.replace(" ", "").chars() {
            if !char.is_numeric(){
                errors
                .entry("mobile_phone".to_string())
                .and_modify(|mobile_phone| {
                    format!(
                        "{} {}",
                        mobile_phone,
                        "Please use only numbers, no punctation.".to_string()
                    );
                })
                .or_insert("Please use only numbers, no punctation.".to_string());
            }
        }
        if (mobile_phone.trim().chars().nth(0) != "0".chars().nth(0))
            || (mobile_phone.trim().chars().nth(0) == "+".chars().nth(0))
        {
            //use the australian not international format!
            errors
                .entry("mobile_phone".to_string())
                .and_modify(|mobile_phone| {
                    format!(
                        "{} {}",
                        mobile_phone,
                        "Use the Australian format, not the international format.".to_string()
                    );
                })
                .or_insert("Use the Australian format, not the international format.".to_string());
        }  
    }
    if address == "" {
        errors.insert(
            ("address").to_string(),
            ("This cannot be left blank!").to_string(),
        );
    }

    return errors;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn check_first_name(first_name: &str) -> String {
    let mut errors: String = "".to_owned();
    println!("{errors}");

    if first_name == "" {
        errors = errors + "This field cannot be left blank";
    };
    errors
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_form_data,
            check_first_name,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
