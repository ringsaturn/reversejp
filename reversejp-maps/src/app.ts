import maplibregl from "maplibre-gl";
import { layers, namedFlavor } from "@protomaps/basemaps";
import init, { initialize, find_properties } from "reversejp-wasm";

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
      sprite: "https://protomaps.github.io/basemaps-assets/sprites/v4/light",
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
      layers: layers("protomaps", namedFlavor("light"), { lang: "ja" }),
    },
  });

  // Add navigation controls
  map.addControl(new maplibregl.NavigationControl(), "top-right");

  // Store markers
  const markers: maplibregl.Marker[] = [];

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

// Display location information in the panel
function displayInfo(
  lng: number,
  lat: number,
  properties: LocationProperties
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
                            ? `<strong>English Name:</strong> ${escapeHtml(
                                prop.enName
                              )}<br>`
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
            <div class="no-data">Error: ${escapeHtml(
              error.message || String(error)
            )}</div>
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

// Initialize everything
(async () => {
  await initWasm();
  initMap();
  setupCloseButton();
})();
