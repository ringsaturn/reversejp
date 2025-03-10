#![doc = include_str!("../README.md")]

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use zip::read::ZipArchive;

use geometry_rs::{Point, Polygon};
use serde::{Deserialize, Serialize};

// Embedded ZIP files
const EMBEDDED_CLASS10S_DATA: &[u8] = include_bytes!("../data/class10s.json.zip");
const EMBEDDED_LANDSLIDES_0_DATA: &[u8] = include_bytes!("../data/landslides_0.json.zip");
const EMBEDDED_LANDSLIDES_1_DATA: &[u8] = include_bytes!("../data/landslides_1.json.zip");
const EMBEDDED_LANDSLIDES_2_DATA: &[u8] = include_bytes!("../data/landslides_2.json.zip");
const EMBEDDED_LANDSLIDES_3_DATA: &[u8] = include_bytes!("../data/landslides_3.json.zip");
const EMBEDDED_LANDSLIDES_4_DATA: &[u8] = include_bytes!("../data/landslides_4.json.zip");
const EMBEDDED_LANDSLIDES_5_DATA: &[u8] = include_bytes!("../data/landslides_5.json.zip");
const EMBEDDED_LANDSLIDES_6_DATA: &[u8] = include_bytes!("../data/landslides_6.json.zip");
const EMBEDDED_LANDSLIDES_7_DATA: &[u8] = include_bytes!("../data/landslides_7.json.zip");
const EMBEDDED_LANDSLIDES_8_DATA: &[u8] = include_bytes!("../data/landslides_8.json.zip");
const EMBEDDED_LANDSLIDES_9_DATA: &[u8] = include_bytes!("../data/landslides_9.json.zip");

// Function to extract JSON from zip data
fn extract_json_from_zip(
    zip_data: &[u8],
    filename: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor)?;
    let mut file = archive.by_name(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Helper function to get class10s data
pub fn get_class10s_data() -> Result<String, Box<dyn std::error::Error>> {
    extract_json_from_zip(EMBEDDED_CLASS10S_DATA, "class10s.json")
}

// Helper function to get landslide data for a specific index
pub fn get_landslide_data(idx: usize) -> Result<String, Box<dyn std::error::Error>> {
    let zip_data = match idx {
        0 => EMBEDDED_LANDSLIDES_0_DATA,
        1 => EMBEDDED_LANDSLIDES_1_DATA,
        2 => EMBEDDED_LANDSLIDES_2_DATA,
        3 => EMBEDDED_LANDSLIDES_3_DATA,
        4 => EMBEDDED_LANDSLIDES_4_DATA,
        5 => EMBEDDED_LANDSLIDES_5_DATA,
        6 => EMBEDDED_LANDSLIDES_6_DATA,
        7 => EMBEDDED_LANDSLIDES_7_DATA,
        8 => EMBEDDED_LANDSLIDES_8_DATA,
        9 => EMBEDDED_LANDSLIDES_9_DATA,
        _ => return Err("Invalid landslide index".into()),
    };

    extract_json_from_zip(zip_data, &format!("landslides_{}.json", idx))
}

// GeoJSON types for deserialization
#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureCollection {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub coordinates: Vec<Vec<Vec<[f64; 2]>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Properties {
    pub code: String,
    pub name: String,
    #[serde(rename = "enName", default)]
    pub en_name: String,
}

// Main struct for reverse geocoding
pub struct ReverseJp {
    polygons: Vec<(Polygon, Properties)>,
}

impl Default for ReverseJp {
    fn default() -> Self {
        Self::new()
    }
}

impl ReverseJp {
    /// Create a new instance with no data
    pub fn new() -> Self {
        ReverseJp {
            polygons: Vec::new(),
        }
    }

    /// Create a new instance with embedded GeoJSON data
    ///
    /// This is the recommended way to use the library as it doesn't require
    /// downloading and managing external data files.
    pub fn with_embedded_data() -> Result<Self, Box<dyn Error>> {
        let mut reverse_jp = Self::new();

        // Load embedded data
        reverse_jp.load_from_str(get_class10s_data()?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(0)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(1)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(2)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(3)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(4)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(5)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(6)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(7)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(8)?.as_str())?;
        reverse_jp.load_from_str(get_landslide_data(9)?.as_str())?;

        Ok(reverse_jp)
    }

    /// Load data from a GeoJSON string
    fn load_from_str(&mut self, json_str: &str) -> Result<(), Box<dyn Error>> {
        let feature_collection: FeatureCollection = serde_json::from_str(json_str)?;
        self.process_feature_collection(feature_collection)
    }

    // Process a feature collection by converting GeoJSON to polygons
    fn process_feature_collection(
        &mut self,
        feature_collection: FeatureCollection,
    ) -> Result<(), Box<dyn Error>> {
        for feature in feature_collection.features {
            if feature.geometry.geometry_type == "MultiPolygon" {
                for multi_polygon in &feature.geometry.coordinates {
                    for polygon_coords in multi_polygon {
                        // Convert GeoJSON coordinates to geometry-rs points
                        let points: Vec<Point> = polygon_coords
                            .iter()
                            .map(|coord| Point {
                                x: coord[0],
                                y: coord[1],
                            })
                            .collect();

                        // Create geometry-rs polygon
                        let polygon = Polygon::new(points, vec![]);
                        self.polygons.push((polygon, feature.properties.clone()));
                    }
                }
            }
        }

        Ok(())
    }

    /// Find all properties for a given longitude/latitude coordinate
    ///
    /// This method returns all properties (regions) that contain the specified point.
    ///
    /// # Arguments
    ///
    /// * `longitude` - The longitude coordinate
    /// * `latitude` - The latitude coordinate
    ///
    /// # Returns
    ///
    /// A vector of Properties for all regions containing the point
    pub fn find_properties(&self, longitude: f64, latitude: f64) -> Vec<Properties> {
        for lng_shift in [0.0, 0.001, -0.001, 0.002, -0.002, 0.005, -0.005] {
            for lat_shift in [0.0, 0.001, -0.001, 0.002, -0.002, 0.005, -0.005] {
                let point = Point {
                    x: longitude + lng_shift,
                    y: latitude + lat_shift,
                };

                // Find all polygons that contain the point
                let properties: Vec<Properties> = self
                    .polygons
                    .iter()
                    .filter(|(polygon, _)| polygon.contains_point(point))
                    .map(|(_, props)| props.clone())
                    .collect();
                if !properties.is_empty() {
                    return properties;
                }
            }
        }
        vec![]
    }

    /// Find all properties for a given longitude/latitude coordinate, return as hashmap
    ///
    /// This method returns all properties (regions) that contain the specified point,
    /// organized in a HashMap with region codes as keys.
    ///
    /// # Arguments
    ///
    /// * `longitude` - The longitude coordinate
    /// * `latitude` - The latitude coordinate
    ///
    /// # Returns
    ///
    /// A HashMap with region codes as keys and Properties as values
    pub fn find_properties_as_hashmap(
        &self,
        longitude: f64,
        latitude: f64,
    ) -> HashMap<String, Properties> {
        let results: Vec<Properties> = self.find_properties(longitude, latitude);
        let mut map = HashMap::new();

        for props in results {
            map.insert(props.code.clone(), props);
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;
    use rand::random_range;

    // Initialize JP_CITIES only once for better performance
    lazy_static! {
        static ref JP_CITIES: Vec<&'static cities_json::City> = {
            cities_json::CITIES
                .iter()
                .filter(|city| city.country == "JP")
                .collect()
        };
    }

    /// Utility function to get a random Japanese city from the cities-json crate.
    fn get_random_jp_city() -> Option<&'static cities_json::City> {
        if JP_CITIES.is_empty() {
            return None;
        }

        // Get a random index
        let random_index = random_range(0..JP_CITIES.len());

        // Return the random city
        Some(JP_CITIES[random_index])
    }

    #[test]
    fn test_get_random_jp_city() {
        let city = get_random_jp_city();
        assert!(
            city.is_some(),
            "Should be able to get a random Japanese city"
        );
        if let Some(city) = city {
            assert_eq!(city.country, "JP", "City should be in Japan");
            println!(
                "Random JP city: {} at ({}, {})",
                city.name, city.lat, city.lng
            );
        }
    }

    #[test]
    fn test_polygon_contains_point() {
        let poly = Polygon::new(
            vec![
                Point {
                    x: 90.48826291293898,
                    y: 45.951129815858565,
                },
                Point {
                    x: 90.48826291293898,
                    y: 27.99437617512571,
                },
                Point {
                    x: 122.83201291294,
                    y: 27.99437617512571,
                },
                Point {
                    x: 122.83201291294,
                    y: 45.951129815858565,
                },
                Point {
                    x: 90.48826291293898,
                    y: 45.951129815858565,
                },
            ],
            vec![],
        );

        let p_out = Point {
            x: 130.74216916294148,
            y: 37.649011392900306,
        };

        let p_in = Point {
            x: 99.9804504129416,
            y: 39.70716466970461,
        };

        assert!(!poly.contains_point(p_out));
        assert!(poly.contains_point(p_in));
    }

    #[test]
    fn test_new_reverse_jp() {
        let reverse_jp = ReverseJp::new();
        assert_eq!(reverse_jp.polygons.len(), 0);
    }

    #[test]
    fn test_find_properties_empty() {
        let reverse_jp = ReverseJp::new();
        let properties = reverse_jp.find_properties(139.7670, 35.6812);
        assert_eq!(properties.len(), 0);
    }

    #[test]
    fn test_find_properties_as_hashmap_empty() {
        let reverse_jp = ReverseJp::new();
        let properties = reverse_jp.find_properties_as_hashmap(139.7670, 35.6812);
        assert_eq!(properties.len(), 0);
    }

    #[test]
    fn test_with_embedded_data() {
        let reverse_jp = ReverseJp::with_embedded_data().unwrap();
        assert!(!reverse_jp.polygons.is_empty());

        // Test Tokyo coordinates
        let properties = reverse_jp.find_properties(139.7670, 35.6812);
        assert!(!properties.is_empty());

        // Check if we can find Tokyo
        let found_tokyo = properties
            .iter()
            .any(|p| p.name == "東京都" || p.en_name == "Tokyo");
        assert!(found_tokyo);
    }

    #[test]
    fn test_all_jp_cities_included() {
        // Get all Japanese cities from the cities-json crate
        let jp_cities: Vec<&cities_json::City> = cities_json::CITIES
            .iter()
            .filter(|city| city.country == "JP")
            .collect();

        // Ensure we have a non-zero number of Japanese cities
        assert!(
            !jp_cities.is_empty(),
            "No Japanese cities found in the cities-json crate"
        );
        println!("Found {} Japanese cities to test", jp_cities.len());

        // Create a new ReverseJp instance with embedded data
        let reverse_jp = ReverseJp::with_embedded_data().unwrap();

        // Test each Japanese city
        let mut found_count = 0;
        let mut missing_cities = Vec::new();

        for city in &jp_cities {
            let properties = reverse_jp.find_properties(city.lng, city.lat);
            if !properties.is_empty() {
                found_count += 1;
            } else {
                missing_cities.push(format!("{} ({},{})", city.name, city.lng, city.lat));
            }
        }

        // Print results
        println!(
            "Found geographical data for {}/{} Japanese cities",
            found_count,
            jp_cities.len()
        );

        if !missing_cities.is_empty() {
            println!("Missing cities: {}", missing_cities.join(", "));
        }

        // Assert that all Japanese cities are found (or a high percentage)
        let coverage_percentage = (found_count as f64 / jp_cities.len() as f64) * 100.0;
        assert!(
            coverage_percentage > 90.0,
            "Only {:.2}% of Japanese cities are covered, expected at least 90%",
            coverage_percentage
        );
    }
}
