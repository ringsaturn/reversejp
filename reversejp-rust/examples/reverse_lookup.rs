use reversejp::ReverseJp;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating ReverseJp instance with embedded data...");

    // Create a new ReverseJp instance with embedded data
    let reverse_jp = ReverseJp::with_embedded_data()?;
    println!("Data loaded successfully!");

    // Test some coordinates
    let test_coordinates = [
        (140.7473, 40.8244),
        // Tokyo
        (139.7670, 35.6812),
        // Kyoto
        (135.7681, 35.0116),
        // Fukuoka
        (130.4219, 33.6063),
        // 台東区
        (139.7864, 35.6972),
    ];

    for (i, (lon, lat)) in test_coordinates.iter().enumerate() {
        println!("\nTest coordinate {}: ({}, {})", i + 1, lon, lat);

        let properties = reverse_jp.find_properties(*lon, *lat);

        if properties.is_empty() {
            println!("No region found for this coordinate");
        } else {
            println!("Found {} regions:", properties.len());

            for prop in properties {
                println!(
                    "  Code: {}, Name: {}, English Name: {}",
                    prop.code, prop.name, prop.en_name
                );
            }
        }
    }

    // Demonstrate using the hashmap version
    println!("\nUsing find_properties_as_hashmap for Tokyo (139.7670, 35.6812):");
    let properties_map = reverse_jp.find_properties_as_hashmap(139.7670, 35.6812);

    for (code, prop) in properties_map {
        println!(
            "  Code: {}, Name: {}, English Name: {}",
            code, prop.name, prop.en_name
        );
    }

    Ok(())
}
