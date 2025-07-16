# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library for reverse geocoding in Japan with Python bindings. Given longitude/latitude coordinates, it returns region information including administrative codes, Japanese names, and English names. The project uses embedded GeoJSON data from the Japan Meteorological Agency.

## Project Structure

- **reversejp-rust/**: Core Rust library implementation
- **reversejp-python/**: Python bindings using PyO3 and maturin
- **Root**: Cargo workspace configuration and project-level commands

## Common Commands

### Building
```bash
# Build entire project (Rust + Python)
make build

# Build Rust library only
cd reversejp-rust && make build

# Build Python package only
cd reversejp-python && make test
```

### Testing
```bash
# Run all tests
make test

# Run Rust tests only
cd reversejp-rust && make test

# Run Python tests only
cd reversejp-python && make test
```

### Linting and Formatting
```bash
# Lint all code
make lint

# Format all code
make fmt
```

### Data Management
```bash
# Download latest GeoJSON data from JMA
make download-data
```

## Architecture

### Core Library (reversejp-rust)
- **ReverseJp**: Main struct that loads embedded GeoJSON data and performs point-in-polygon queries
- **Embedded Data**: GeoJSON data compressed as ZIP files and embedded at compile time
- **Data Sources**: 
  - class10s.json: Administrative regions
  - landslides_*.json: Landslide risk areas (10 files)
  - Other JSON files for various geographical features

### Python Bindings (reversejp-python)
- Uses PyO3 to expose Rust functionality to Python
- Global singleton pattern for efficient data loading
- Maturin for building Python wheels

### Data Processing
- GeoJSON features are converted to geometry-rs polygons
- Point-in-polygon queries use spatial indexing
- Coordinates are shifted slightly to handle edge cases

## Key Technical Details

- Uses geometry-rs for polygon operations
- Data is embedded as compressed ZIP files to reduce binary size
- Python bindings use a global static instance for performance
- Supports both direct property lookup and HashMap-based results
- Includes comprehensive test coverage with Japanese cities data

## Development Workflow

1. Modify Rust code in `reversejp-rust/src/lib.rs`
2. Update Python bindings in `reversejp-python/src/lib.rs` if needed
3. Run tests to ensure functionality
4. Use `make lint` and `make fmt` before committing
5. Build both packages with `make build`

## Data Updates

The `scripts/download.py` script fetches latest data from JMA APIs. Run `make download-data` to update the embedded GeoJSON files.