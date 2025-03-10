use reversejp::ReverseJp;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("ReverseJp - Loading All Landslide Files Example");

    // Method 1: Using embedded data (recommended)
    println!("\nMethod 1: Using embedded data (includes all landslide files)");
    let reverse_jp_embedded = ReverseJp::with_embedded_data()?;
    println!("Embedded data loaded successfully!");

    // Test a coordinate with the embedded data instance
    let (lon, lat) = (139.7670, 35.6812); // Tokyo
    println!(
        "\nTesting coordinate at Tokyo ({}, {}) with embedded data:",
        lon, lat
    );
    let properties = reverse_jp_embedded.find_properties(lon, lat);

    if !properties.is_empty() {
        println!("Found {} regions:", properties.len());
        for prop in properties {
            println!(
                "  Code: {}, Name: {}, English Name: {}",
                prop.code, prop.name, prop.en_name
            );
        }
    } else {
        println!("No regions found for this coordinate with embedded data");
    }

    Ok(())
}
