#!/usr/bin/env python3
"""
Simple example of using the reversejp Python binding.
For a more complete example, see example.py.
"""

from argparse import ArgumentParser

import reversejp


def main():
    # # Tokyo coordinates
    # longitude = 139.7673068
    # latitude = 35.6809591

    parser = ArgumentParser()
    parser.add_argument("--longitude", type=float, required=True)
    parser.add_argument("--latitude", type=float, required=True)
    args = parser.parse_args()

    # Find all properties for Tokyo
    properties = reversejp.find_properties(args.longitude, args.latitude)

    print(f"Found {len(properties)} regions for {args.longitude}, {args.latitude}:")
    for prop in properties:
        print(f"  {prop.code}: {prop.name}")


if __name__ == "__main__":
    main()
