import maplibregl from "maplibre-gl";
import { layers, namedFlavor } from "@protomaps/basemaps";
import init, {
  find_properties,
  get_landslide_data_wasm,
  initialize,
} from "reversejp-wasm";

// Type definitions for the WASM module
interface Property {
  code: string;
  name: string;
  enName: string;
}

type LocationProperties = Property[];

// Initialize the WASM module
let wasmInitialized = false;

async function initWasm(): Promise<void> {
  try {
    await init();
    initialize();
    wasmInitialized = true;
    console.log("✅ ReverseJP WASM initialized");
  } catch (error) {
    console.error("❌ Failed to initialize WASM:", error);
  }
}

// Initialize the map
function initMap(): maplibregl.Map {
  const PROTOMAPS_API_KEY = "9bbd0c5d1011a6bd";

  // Create the map centered on Japan using Protomaps hosted API
  // Japan's approximate bounding box: [west, south, east, north]
  const japanBounds: maplibregl.LngLatBoundsLike = [
    [122.0, 24.0], // Southwest coordinates (Yonaguni Island area)
    [154.0, 46.0], // Northeast coordinates (Northern Hokkaido, Kuril area)
  ];

  const map = new maplibregl.Map({
    container: "map",
    center: [139.767, 35.6812], // Tokyo coordinates
    zoom: 5,
    maxBounds: japanBounds, // Restrict map to Japan's region
    style: {
      version: 8,
      glyphs:
        "https://protomaps.github.io/basemaps-assets/fonts/{fontstack}/{range}.pbf",
      sprite: "https://protomaps.github.io/basemaps-assets/sprites/v4/white",
      sources: {
        protomaps: {
          type: "vector",
          tiles: [
            `https://api.protomaps.com/tiles/v4/{z}/{x}/{y}.mvt?key=${PROTOMAPS_API_KEY}`,
          ],
          maxzoom: 15,
          attribution:
            '<a href="https://protomaps.com">Protomaps</a> © <a href="https://openstreetmap.org">OpenStreetMap</a>',
        },
      },
      layers: layers("protomaps", namedFlavor("white"), { lang: "ja" }),
    },
  });

  // Add navigation controls
  map.addControl(new maplibregl.NavigationControl(), "top-right");

  // Store markers
  const markers: maplibregl.Marker[] = [];

  // Store the current map reference for external access
  let mapInstance = map;

  // Handle map clicks
  map.on("click", async (e: maplibregl.MapMouseEvent) => {
    if (!wasmInitialized) {
      alert("Please wait, the geocoding system is still loading...");
      return;
    }

    const { lng, lat } = e.lngLat;

    try {
      // Call the WASM function to get location properties
      const properties = find_properties(lng, lat) as LocationProperties;

      // Remove previous markers
      markers.forEach((marker) => marker.remove());
      markers.length = 0;

      // Add a new marker
      const markerElement = document.createElement("div");
      markerElement.className = "marker";

      const marker = new maplibregl.Marker({
        element: markerElement,
        anchor: "bottom",
      })
        .setLngLat([lng, lat])
        .addTo(map);

      markers.push(marker);

      // Display the information
      displayInfo(lng, lat, properties);
    } catch (error) {
      console.error("Error getting location properties:", error);
      displayError(lng, lat, error as Error);
    }
  });

  return map;
}

// Load and display landslide GeoJSON data
function loadLandslideData(
  map: maplibregl.Map,
  idx: number,
  fitBounds: boolean = true,
): void {
  try {
    // Get GeoJSON string from WASM
    const geoJsonString = get_landslide_data_wasm(idx) as string;
    const geoJsonData = JSON.parse(geoJsonString);

    const sourceId = `landslide-source-${idx}`;
    const layerId = `landslide-layer-${idx}`;
    const outlineLayerId = `landslide-outline-${idx}`;

    // Remove existing layers and source if they exist
    if (map.getLayer(outlineLayerId)) {
      map.removeLayer(outlineLayerId);
    }
    if (map.getLayer(layerId)) {
      map.removeLayer(layerId);
    }
    if (map.getSource(sourceId)) {
      map.removeSource(sourceId);
    }

    // Add the GeoJSON source
    map.addSource(sourceId, {
      type: "geojson",
      data: geoJsonData,
    });

    // Use different colors for different indices
    const colors = [
      "#ff0000", // Red
      "#ff6b00", // Orange-red
      "#ff9500", // Orange
      "#ffbb00", // Yellow-orange
      "#ffd700", // Gold
      "#00ff00", // Green
      "#00bfff", // Sky blue
      "#0080ff", // Blue
      "#4169e1", // Royal blue
      "#8b00ff", // Purple
    ];

    // Add a fill layer for polygons
    map.addLayer({
      id: layerId,
      type: "fill",
      source: sourceId,
      paint: {
        "fill-color": colors[idx] || "#ff0000",
        "fill-opacity": 0.3,
      },
    });

    // Add an outline layer
    map.addLayer({
      id: outlineLayerId,
      type: "line",
      source: sourceId,
      paint: {
        "line-color": colors[idx] || "#ff0000",
        "line-width": 1.5,
      },
    });

    // Fit map to the bounds of the GeoJSON data (only if requested)
    if (fitBounds && geoJsonData.features && geoJsonData.features.length > 0) {
      const bounds = new maplibregl.LngLatBounds();

      geoJsonData.features.forEach((feature: any) => {
        if (feature.geometry.type === "Polygon") {
          feature.geometry.coordinates[0].forEach((coord: number[]) => {
            bounds.extend(coord as [number, number]);
          });
        } else if (feature.geometry.type === "MultiPolygon") {
          feature.geometry.coordinates.forEach((polygon: number[][][]) => {
            if (polygon[0]) {
              polygon[0].forEach((coord: number[]) => {
                bounds.extend(coord as [number, number]);
              });
            }
          });
        }
      });

      map.fitBounds(bounds, { padding: 50 });
    }

    console.log(`✅ Loaded landslide data ${idx}`);
  } catch (error) {
    console.error(`❌ Failed to load landslide data ${idx}:`, error);
    // Don't show alert for auto-load failures
  }
}

// Load all landslide data automatically
function loadAllLandslideData(map: maplibregl.Map): void {
  console.log("🔄 Auto-loading all landslide data...");
  for (let i = 0; i < 10; i++) {
    try {
      loadLandslideData(map, i, false); // Don't fit bounds for each layer
    } catch (error) {
      console.error(`Failed to auto-load landslide data ${i}:`, error);
    }
  }
  console.log("✅ Finished auto-loading landslide data");
}

// Clear all landslide layers
function clearLandslideLayers(map: maplibregl.Map): void {
  // Remove layers 0-9 (adjust range if needed)
  for (let i = 0; i < 10; i++) {
    const sourceId = `landslide-source-${i}`;
    const layerId = `landslide-layer-${i}`;
    const outlineLayerId = `landslide-outline-${i}`;

    if (map.getLayer(outlineLayerId)) {
      map.removeLayer(outlineLayerId);
    }
    if (map.getLayer(layerId)) {
      map.removeLayer(layerId);
    }
    if (map.getSource(sourceId)) {
      map.removeSource(sourceId);
    }
  }
  console.log("✅ Cleared all landslide layers");
}

// Display location information in the panel
function displayInfo(
  lng: number,
  lat: number,
  properties: LocationProperties,
): void {
  const infoPanel = document.getElementById("info-panel");
  const infoContent = document.getElementById("info-content");

  if (!infoPanel || !infoContent) return;

  let html = `
        <div class="info-section">
            <div class="info-label">Coordinates</div>
            <div class="coordinates">
                Latitude: ${lat.toFixed(6)}<br>
                Longitude: ${lng.toFixed(6)}
            </div>
        </div>
    `;

  // Display properties if available
  if (properties && properties.length > 0) {
    html += `
            <div class="info-section">
                <div class="info-label">Location Properties</div>
            </div>
        `;

    // Display each property
    properties.forEach((prop, index) => {
      html += `
                <div class="info-section">
                    <div class="info-label">Region ${index + 1}</div>
                    <div class="info-value">
                        <strong>Name:</strong> ${escapeHtml(prop.name)}<br>
                        ${
        prop.enName
          ? `<strong>English Name:</strong> ${
            escapeHtml(
              prop.enName,
            )
          }<br>`
          : ""
      }
                        <strong>Code:</strong> ${escapeHtml(prop.code)}
                    </div>
                </div>
            `;
    });
  } else {
    html += `
            <div class="info-section">
                <div class="no-data">No location data available for this point.</div>
            </div>
        `;
  }

  infoContent.innerHTML = html;
  infoPanel.classList.remove("hidden");
}

// Display error message
function displayError(lng: number, lat: number, error: Error): void {
  const infoPanel = document.getElementById("info-panel");
  const infoContent = document.getElementById("info-content");

  if (!infoPanel || !infoContent) return;

  const html = `
        <div class="info-section">
            <div class="info-label">Coordinates</div>
            <div class="coordinates">
                Latitude: ${lat.toFixed(6)}<br>
                Longitude: ${lng.toFixed(6)}
            </div>
        </div>
        <div class="info-section">
            <div class="no-data">Error: ${
    escapeHtml(
      error.message || String(error),
    )
  }</div>
        </div>
    `;

  infoContent.innerHTML = html;
  infoPanel.classList.remove("hidden");
}

// Close button handler
function setupCloseButton(): void {
  const closeBtn = document.getElementById("close-btn");
  const infoPanel = document.getElementById("info-panel");

  if (closeBtn && infoPanel) {
    closeBtn.addEventListener("click", () => {
      infoPanel.classList.add("hidden");
    });
  }
}

// Utility function to escape HTML
function escapeHtml(text: string): string {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

// Setup landslide data controls
function setupLandslideControls(map: maplibregl.Map): void {
  const controlsContainer = document.getElementById("landslide-controls");
  if (!controlsContainer) return;

  // Create buttons for landslide data indices 0-9
  for (let i = 0; i < 10; i++) {
    const button = document.createElement("button");
    button.textContent = `${i}`;
    button.className = "landslide-btn";
    button.title = `Toggle landslide data ${i}`;
    button.addEventListener("click", () => {
      if (!wasmInitialized) {
        alert("Please wait, the system is still loading...");
        return;
      }

      // Toggle layer visibility
      const layerId = `landslide-layer-${i}`;
      const outlineLayerId = `landslide-outline-${i}`;

      if (map.getLayer(layerId)) {
        const visibility = map.getLayoutProperty(layerId, "visibility");
        const newVisibility = visibility === "visible" ? "none" : "visible";
        map.setLayoutProperty(layerId, "visibility", newVisibility);
        map.setLayoutProperty(outlineLayerId, "visibility", newVisibility);
        button.classList.toggle("active", newVisibility === "visible");
      } else {
        loadLandslideData(map, i, true);
        button.classList.add("active");
      }
    });
    // Don't mark as active by default since layers aren't auto-loaded
    controlsContainer.appendChild(button);
  }

  // Add a load all button
  const loadAllButton = document.createElement("button");
  loadAllButton.textContent = "Load All";
  loadAllButton.className = "landslide-btn load-all-btn";
  loadAllButton.addEventListener("click", () => {
    if (!wasmInitialized) {
      alert("Please wait, the system is still loading...");
      return;
    }
    loadAllLandslideData(map);
    // Set all buttons to active
    controlsContainer.querySelectorAll(
      ".landslide-btn:not(.clear-btn):not(.load-all-btn)",
    ).forEach((btn) => {
      btn.classList.add("active");
    });
  });
  controlsContainer.appendChild(loadAllButton);

  // Add a clear button
  const clearButton = document.createElement("button");
  clearButton.textContent = "Clear All";
  clearButton.className = "landslide-btn clear-btn";
  clearButton.addEventListener("click", () => {
    clearLandslideLayers(map);
    // Reset button states
    controlsContainer.querySelectorAll(
      ".landslide-btn:not(.clear-btn):not(.load-all-btn)",
    ).forEach((btn) => {
      btn.classList.remove("active");
    });
  });
  controlsContainer.appendChild(clearButton);
}

// Initialize everything
(async () => {
  await initWasm();
  const map = initMap();
  setupCloseButton();
  setupLandslideControls(map);
})();
