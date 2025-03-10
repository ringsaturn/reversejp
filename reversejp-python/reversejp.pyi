from typing import Dict, List

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

def find_properties_as_dict(longitude: float, latitude: float) -> Dict[str, Property]:
    """
    Find all properties (regions) that contain the specified longitude/latitude coordinate,
    returned as a dictionary.

    Args:
        longitude: The longitude coordinate
        latitude: The latitude coordinate

    Returns:
        A dictionary with region codes as keys and Property objects as values
    """
    ...
