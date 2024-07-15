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

    fn num_allocations(&self) -> usize {
        self.allocations.len()
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

    fn get_target(&self, name: &str) -> &Option<Decimal> {
        &self
            .allocations
            .get(name)
            .expect("Unable to retrieve allocation")
            .pct_target
    }

    fn set_allocation(&mut self, name: String, amount: Decimal) {
        // NOTE: I think we always know that the key is here so maybe we don't need to use the
        // entry API. On the other hand, it probably doesn't hurt?
        self.allocations
            .entry(name)
            .and_modify(|e| e.amount = amount);
    }

    fn set_target(&mut self, name: String, target: Decimal) {
        self.allocations
            .entry(name)
            .and_modify(|e| e.pct_target = Some(target));
    }

    fn complete(&self) -> bool {
        // Do we have an allocation set for all our asset classes?
        self.allocations.values().all(|v| v.pct_target.is_some())
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

        if portfolio.num_allocations() > 0 {
            // If we have at least one asset class, we can start setting allocations and targets
            options.push("Set Allocation");
            options.push("Set Target");
            // We can also check on our current Portfolio
            options.push("Check Portfolio");

            if portfolio.complete() {
                // If we have targets set for all the asset classes, we can compute a plan
                options.push("Compute Exchange");
            }
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
            Ok("Set Target") => {
                let asset_class =
                    Select::new("Choose Asset Class:", portfolio.asset_classes()).prompt();

                match asset_class {
                    Ok(asset_class) => {
                        let current_target = match portfolio.get_target(&asset_class) {
                            Some(target) => format!("{}", target),
                            None => String::from("Unspecified"),
                        };

                        let prompt = format!(
                            "New Target for {} (Current: {}):",
                            asset_class, current_target
                        );

                        let new_target = Text::new(&prompt).prompt();

                        match new_target {
                            Ok(new_target) => portfolio.set_target(
                                asset_class.to_string(),
                                Decimal::from_str(new_target.as_str()).unwrap(),
                            ),
                            Err(_) => println!(
                                "An error happened when trying to set the target, please try again"
                            ),
                        }
                    }
                    Err(_) => println!("There was an error, please try again"),
                }
            }
            Ok("Check Portfolio") => {
                println!("{}", portfolio.summary_table());
            }
            Ok("Compute Exchange") => {
                for (asset_class, allocation) in &portfolio.allocations {
                    let ideal_amount = portfolio.total()
                        * (allocation.pct_target.expect("Unexpected unset target") / dec!(100));

                    let adjustment = ideal_amount - allocation.amount;

                    if adjustment > dec!(0.0) {
                        println!("Buy ${:.2} of {}", adjustment, asset_class);
                    } else {
                        println!("Sell ${:.2} of {}", -adjustment, asset_class);
                    }
                }
            }
            Ok("Exit") => break,
            Ok(_) => unimplemented!(),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    Ok(())
}
