use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lazy_static::lazy_static;
use rand::random_range;
use reversejp::ReverseJp;

// Initialize JP_CITIES only once for better performance
lazy_static! {
    static ref JP_CITIES: Vec<&'static cities_json::City> = {
        cities_json::CITIES
            .iter()
            .filter(|city| city.country == "JP")
            .collect()
    };
}

/// Utility function to get a random Japanese city from the cities-json crate
fn get_random_jp_city() -> Option<&'static cities_json::City> {
    if JP_CITIES.is_empty() {
        return None;
    }

    // Get a random index
    let random_index = random_range(0..JP_CITIES.len());

    // Return the random city
    Some(JP_CITIES[random_index])
}

fn benchmark_jp_city_lookup(c: &mut Criterion) {
    // Initialize ReverseJp with embedded data (do this outside the benchmark loop)
    let reverse_jp = ReverseJp::with_embedded_data().unwrap();

    // Create a benchmark group for city lookups
    let mut group = c.benchmark_group("jp_city_lookups");

    // Benchmark single city lookup
    group.bench_function("single_random_city", |b| {
        b.iter(|| {
            if let Some(city) = get_random_jp_city() {
                black_box(reverse_jp.find_properties(city.lng, city.lat))
            } else {
                // Fallback to Tokyo coordinates if no cities found
                black_box(reverse_jp.find_properties(139.7670, 35.6812))
            }
        })
    });

    group.finish();
}

// Measure performance distribution for a large number of random cities
fn benchmark_distribution(c: &mut Criterion) {
    let reverse_jp = ReverseJp::with_embedded_data().unwrap();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("jp_city_performance_distribution");

    // Run the benchmark multiple times to see performance distribution
    for i in 0..2 {
        group.bench_with_input(BenchmarkId::new("random_sample", i), &i, |b, _| {
            b.iter(|| {
                if let Some(city) = get_random_jp_city() {
                    black_box(reverse_jp.find_properties(city.lng, city.lat))
                } else {
                    // Fallback to Tokyo coordinates if no cities found
                    black_box(reverse_jp.find_properties(139.7670, 35.6812))
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_jp_city_lookup, benchmark_distribution);
criterion_main!(benches);
