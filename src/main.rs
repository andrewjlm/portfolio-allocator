use std::collections::{HashMap, HashSet};
use std::io;

use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use inquire::{
    error::{InquireError, InquireResult},
    Select, Text,
};

#[derive(Debug)]
struct Allocation {
    amount: Decimal,
    pct_target: Option<Decimal>,
}

#[derive(Debug)]
struct Portfolio {
    allocations: HashMap<String, Allocation>,
}

impl Portfolio {
    fn new() -> Portfolio {
        Portfolio {
            allocations: HashMap::new(),
        }
    }

    fn total(&self) -> Decimal {
        self.allocations.values().map(|v| v.amount).sum()
    }

    fn add_asset_class(&mut self, name: String) {
        // Add a new asset class, defaulting to an allocation of zero
        self.allocations.insert(
            name.clone(),
            Allocation {
                amount: dec!(0.00),
                pct_target: None,
            },
        );
    }

    fn asset_classes(&self) -> Vec<&str> {
        self.allocations.keys().map(AsRef::as_ref).collect()
    }

    fn get_allocation(&self, name: &str) -> &Decimal {
        &self
            .allocations
            .get(name)
            .expect("Unable to retrieve allocation")
            .amount
    }

    fn set_allocation(&mut self, name: String, amount: Decimal) {
        // NOTE: I think we always know that the key is here so maybe we don't need to use the
        // entry API. On the other hand, it probably doesn't hurt?
        self.allocations
            .entry(name)
            .and_modify(|e| e.amount = amount);
    }

    fn summary_table(&self) -> String {
        let mut result = String::from("Asset Class\tAllocation\tTarget");
        for (asset_class, allocation) in &self.allocations {
            let row = format!(
                "\n{}\t{}\t{:?}",
                asset_class, allocation.amount, allocation.pct_target
            );
            result.push_str(row.as_str());
        }

        result
    }
}

fn main() -> InquireResult<()> {
    let mut portfolio = Portfolio::new();

    loop {
        let mut options: Vec<&str> = vec!["Add Asset Class", "Exit"];

        if portfolio.allocations.len() > 0 {
            // If we have at least one asset class, we can start setting allocations and targets
            options.push("Set Allocation");
            options.push("Set Target");
            // We can also check on our current Portfolio
            options.push("Check Portfolio");
        }

        if portfolio.allocations.len() >= 2 {
            // If there are less than two asset classes, we need to make the user add until there
            // are two minimum for our application to do anything
        }

        let ans: Result<&str, InquireError> = Select::new("Command:", options).prompt();

        match ans {
            Ok("Add Asset Class") => {
                println!("Current Asset Classes:\n{:?}", portfolio.allocations.keys());
                let new_asset_class = Text::new("New Asset Class:").prompt();

                match new_asset_class {
                    Ok(new_asset_class) => portfolio.add_asset_class(new_asset_class),
                    Err(_) => println!("Error occurred adding asset class"),
                }
            }
            Ok("Set Allocation") => {
                let asset_class =
                    Select::new("Choose Asset Class:", portfolio.asset_classes()).prompt();

                match asset_class {
                    Ok(asset_class) => {
                        let prompt = format!(
                            "New Allocation for {} (Current: {}):",
                            asset_class,
                            portfolio.get_allocation(&asset_class)
                        );

                        // TODO: Validation? I feel like first we parse as f64, then convert to
                        // Decimal(2)?
                        let new_allocation = Text::new(&prompt).prompt();

                        match new_allocation {
                            Ok(new_allocation) => portfolio.set_allocation(asset_class.to_string(), Decimal::from_str(new_allocation.as_str()).unwrap()),
                            Err(_) => println!("An error happened when trying to set the allocation, please try again")
                        }
                    }
                    Err(_) => println!("There was an error, please try again"),
                }
            }
            Ok("Check Portfolio") => {
                println!("{}", portfolio.summary_table());
            }
            Ok("Exit") => break,
            Ok(_) => unimplemented!(),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    Ok(())
}
