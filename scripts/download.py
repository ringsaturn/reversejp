import zipfile

import requests


def fetch_landslides():
    LANDSLIDES_TPL = (
        "https://www.jma.go.jp/bosai/common/const/geojson/landslides_{idx}.json"
    )
    for idx in range(0, 10):
        url = LANDSLIDES_TPL.format(idx=idx)
        base_name = f"reversejp-rust/data/landslides_{idx}.json.zip"
        content = requests.get(url).content
        with zipfile.ZipFile(base_name, "w", zipfile.ZIP_DEFLATED) as zf:
            zf.writestr(f"landslides_{idx}.json", content)


def fetch_landslide_area():
    url = "https://www.jma.go.jp/bosai/common/const/landslide_area.json"
    with zipfile.ZipFile(
        "reversejp-rust/data/landslide_area.json.zip", "w", zipfile.ZIP_DEFLATED
    ) as zf:
        content = requests.get(url).content
        zf.writestr("landslide_area.json", content)


def fetch_area():
    url = "https://www.jma.go.jp/bosai/common/const/area.json"
    with zipfile.ZipFile(
        "reversejp-rust/data/area.json.zip", "w", zipfile.ZIP_DEFLATED
    ) as zf:
        content = requests.get(url).content
        zf.writestr("area.json", content)


def fetch_warning():
    url = "https://www.jma.go.jp/bosai/const/selectorinfos/warning.json"
    with zipfile.ZipFile(
        "reversejp-rust/data/warning.json.zip", "w", zipfile.ZIP_DEFLATED
    ) as zf:
        content = requests.get(url).content
        zf.writestr("warning.json", content)


def fetch_class10s():
    url = "https://www.jma.go.jp/bosai/common/const/geojson/class10s.json"
    with zipfile.ZipFile(
        "reversejp-rust/data/class10s.json.zip", "w", zipfile.ZIP_DEFLATED
    ) as zf:
        content = requests.get(url).content
        zf.writestr("class10s.json", content)


def fetch_xy():
    url = "https://www.jma.go.jp/bosai/common/const/xy.json"
    with zipfile.ZipFile(
        "reversejp-rust/data/xy.json.zip", "w", zipfile.ZIP_DEFLATED
    ) as zf:
        content = requests.get(url).content
        zf.writestr("xy.json", content)


if __name__ == "__main__":
    fetch_landslides()
    fetch_landslide_area()
    fetch_area()
    fetch_warning()
    fetch_class10s()
    fetch_xy()
