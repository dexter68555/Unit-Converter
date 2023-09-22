use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Unit {
    name: String,
    abbreviation: String,
    category: String,
    conversion_factor: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    name: String,
    units: Vec<Unit>,
}

//Load unit from json file
fn load_units(filename: &str) -> Result<Value, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;
    Ok(data)
}

//convert unit
fn convert_unit(
    value: f64,
    from_unit: &Unit,
    to_unit: &Unit,
    categories: &Vec<Category>,
    category_name: &str,
) -> Result<f64, &'static str> {
    if from_unit.category != to_unit.category {
        return Err("Units are not in the same category.");
    }


    let from_category = categories
        .iter()
        .find(|&category| category.name == category_name)
        .ok_or("Category not found")?;

    let from_unit = from_category
        .units
        .iter()
        .find(|&unit| unit.abbreviation == from_unit.abbreviation)
        .ok_or("Unit not found")?;

    let to_unit = from_category
        .units
        .iter()
        .find(|&unit| unit.abbreviation == to_unit.abbreviation)
        .ok_or("Unit not found")?;

    Ok(value * from_unit.conversion_factor / to_unit.conversion_factor)
}

//display all categories
fn display_categories(categories: &Vec<Category>) {
    for category in categories {
        println!("{}", category.name);
    }
}

//display units
fn display_units(category_name: &str, categories: &Vec<Category>) {
    if let Some(category) = categories.iter().find(|&c| c.name == category_name) {
        for unit in &category.units {
            println!("{} ({})", unit.name, unit.abbreviation);
        }
    } else {
        println!("Category not found.");
    }
}

fn main() -> Result<(), io::Error> {
    let filename = "units.json";
    let data = load_units(filename)?;

    let categories: Vec<Category> = serde_json::from_value(data).unwrap();

    loop {
        println!("Unit Converter");
        println!("Options:");
        println!("1. Convert Unit");
        println!("2. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Available Categories:");
                display_categories(&categories);

                println!("Enter Category Name:");
                let mut category_name = String::new();
                io::stdin().read_line(&mut category_name)?;

                display_units(category_name.trim(), &categories);
                
                println!("Enter Value To Convert:");
                let mut value = String::new();
                io::stdin().read_line(&mut value)?;
                let value: f64 = value.trim().parse().unwrap();

                println!("From What Unit (abbreviation):");
                let mut from_unit = String::new();
                io::stdin().read_line(&mut from_unit)?;

                println!("To What Unit (abbreviation):");
                let mut to_unit = String::new();
                io::stdin().read_line(&mut to_unit)?;

                println!("{category_name}");

                match convert_unit(
                    value,
                    &Unit {
                        name: String::new(),
                        abbreviation: from_unit.trim().to_string(),
                        category: String::new(),
                        conversion_factor: 1.0,
                    },
                    &Unit {
                        name: String::new(),
                        abbreviation: to_unit.trim().to_string(),
                        category: String::new(),
                        conversion_factor: 1.0,
                    },
                    &categories,
                    category_name.trim(),
                ) {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => println!("{}", err),
                }

            }
            "2" => break,
            _ => println!("Invalid option."),
        }
    }

    Ok(())
}
