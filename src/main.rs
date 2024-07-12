use std::collections::HashMap;
use std::io;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug)]
struct Portfolio {
    // TODO: Probably better not to use floats for money...
    total: Decimal,
    allocations: HashMap<String, Decimal>,
}

fn main() {
    let mut current_portfolio = Portfolio {
        total: dec!(0.00),
        allocations: HashMap::new(),
    };

    let mut ideal_allocations = HashMap::new();

    // Input current Portfolio
    println!("Enter your current portfolio: ");
    input_portfolio(&mut current_portfolio);

    // Input ideal allocations
    println!("Enter your ideal allocations (in percentages):");
    input_ideal_allocations(&mut ideal_allocations);

    // Calculate and display adjustments
    calculate_adjustments(&current_portfolio, &ideal_allocations);
}

fn input_portfolio(portfolio: &mut Portfolio) {
    loop {
        println!("Enter asset class (or 'done' to finish):");
        let mut asset_class = String::new();
        io::stdin()
            .read_line(&mut asset_class)
            .expect("Failed to read line");
        let asset_class = asset_class.trim();

        if asset_class == "done" {
            break;
        }

        println!("Enter amount invested in {}:", asset_class);
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        let amount: f64 = amount.trim().parse().expect("Please enter a number");
        let amount_dec = Decimal::from_f64_retain(amount)
            .expect("Failed to convert to Decimal")
            .round_dp(2);

        portfolio
            .allocations
            .insert(asset_class.to_string(), amount_dec);
        portfolio.total += amount_dec;
    }
}

fn input_ideal_allocations(ideal_allocations: &mut HashMap<String, f64>) {
    loop {
        println!("Enter asset class (or 'done' to finish):");
        let mut asset_class = String::new();
        io::stdin()
            .read_line(&mut asset_class)
            .expect("Failed to read line");
        let asset_class = asset_class.trim();

        if asset_class == "done" {
            break;
        }

        println!("Enter ideal percentage for {}:", asset_class);
        let mut percentage = String::new();
        io::stdin()
            .read_line(&mut percentage)
            .expect("Failed to read line");
        let percentage: f64 = percentage.trim().parse().expect("Please enter a number");

        ideal_allocations.insert(asset_class.to_string(), percentage / 100.0);
    }
}

fn calculate_adjustments(current: &Portfolio, ideal: &HashMap<String, f64>) {
    println!("\nAdjustments needed:");

    for (asset_class, ideal_percentage) in ideal {
        let ideal_amount = current.total
            * Decimal::from_f64_retain(*ideal_percentage).expect("Failed to convert to decimal");
        let default_amount = &dec!(0.0);
        let current_amount = current
            .allocations
            .get(asset_class)
            .unwrap_or(default_amount);
        let adjustment = ideal_amount - current_amount;

        if adjustment.abs() > dec!(0.01) {
            // Avoid displaying very small adjustments
            if adjustment > dec!(0.0) {
                println!("Buy ${:.2} of {}", adjustment, asset_class);
            } else {
                println!("Sell ${:.2} of {}", -adjustment, asset_class);
            }
        }
    }
}
