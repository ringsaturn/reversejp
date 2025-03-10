import random

from citiespy import all_cities
from pytest import mark
from reversejp import find_properties

all_jp_cities = [city for city in all_cities() if city.country == "JP"]


def random_jp_city():
    return random.choice(all_jp_cities)


def test_find_properties():
    for city in all_jp_cities:
        properties = find_properties(city.lng, city.lat)
        if properties is None:
            raise Exception(f"No properties found for {city.name}")


@mark.parametrize(
    "lng, lat, expected_count, expected_codes, expected_names",
    [
        (139.7673068, 35.6809591, 2, ["130010", "1310100"], ["東京都", "千代田区"]),
        (139.701402, 35.6585805, 2, ["130010", "1311300"], ["東京都", "渋谷区"]),
        (139.774856, 35.681236, 2, ["130010", "1310200"], ["東京都", "中央区"]),
        (135.5022535, 34.6937378, 2, ["270000", "2710000"], ["大阪府", "大阪市"]),
        (141.3469, 43.0619, 2, ["016010", "0110000"], ["石狩地方", "札幌市"]),
    ],
    ids=[
        "Tokyo",
        "Shibuya",
        "Shinjuku",
        "Osaka",
        "Sapporo",
    ],
)
def test_locs(lng, lat, expected_count, expected_codes, expected_names):
    properties = find_properties(lng, lat)
    assert len(properties) == expected_count
    assert [p.code for p in properties] == expected_codes
    assert [p.name for p in properties] == expected_names


def _test_city():
    city = random_jp_city()
    _ = find_properties(city.lng, city.lat)


def test_city_benchmark(benchmark):
    benchmark(_test_city)
