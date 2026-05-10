fn format_change(cents: u32) -> String {
    let dollars = cents / 100;
    let cents = cents % 100;

    let quarters = cents / 25;
    let cents = cents % 25;

    let dimes = cents / 10;
    let cents = cents % 10;

    let nickels = cents / 5;
    let pennies = cents % 5;

    let mut parts: Vec<String> = Vec::new();

    if dollars > 0 {
        parts.push(format!("{} {}", dollars, if dollars == 1 { "dollar" } else { "dollars" }));
    }
    if quarters > 0 {
        parts.push(format!("{} {}", quarters, if quarters == 1 { "quarter" } else { "quarters" }));
    }
    if dimes > 0 {
        parts.push(format!("{} {}", dimes, if dimes == 1 { "dime" } else { "dimes" }));
    }
    if nickels > 0 {
        parts.push(format!("{} {}", nickels, if nickels == 1 { "nickel" } else { "nickels" }));
    }
    if pennies > 0 {
        parts.push(format!("{} {}", pennies, if pennies == 1 { "penny" } else { "pennies" }));
    }

    if parts.is_empty() {
        return String::from("no change");
    } else if parts.len() == 1 {
        return parts[0].clone();
    } else {
        let last = parts.pop().unwrap();
        return format!("{} and {}", parts.join(", "), last);
    }
}

fn main() {
    println!("{}", format_change(1));
    println!("{}", format_change(15));
    println!("{}", format_change(25));
    println!("{}", format_change(100));
    println!("{}", format_change(141));
    println!("{}", format_change(2));
    println!("{}", format_change(20));
    println!("{}", format_change(50));
    println!("{}", format_change(200));
    println!("{}", format_change(387));
    println!("{}", format_change(176));
    println!("{}", format_change(115));
    println!("{}", format_change(101));
    println!("{}", format_change(300));
    println!("{}", format_change(30));
    println!("{}", format_change(0));
    println!("{}", format_change(100000));
}