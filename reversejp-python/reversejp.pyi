from typing import List

class Property:
    """A class representing a property (region)."""

    code: str
    """The region code."""

    name: str
    """The region name in Japanese."""

    en_name: str
    """The region name in English."""

    def __init__(self, code: str, name: str, en_name: str) -> None: ...

def find_properties(longitude: float, latitude: float) -> List[Property]:
    """
    Find all properties (regions) that contain the specified longitude/latitude coordinate.

    Args:
        longitude: The longitude coordinate
        latitude: The latitude coordinate

    Returns:
        A list of Property objects representing regions that contain the specified point
    """
    ...

def get_landslide_data(idx: int) -> str:
    """
    Get landslide polygon data for the specified region index.

    Args:
        idx: The region index, [0, 9]
    Returns:
        A string representing the landslide polygon data
    """
    ...
