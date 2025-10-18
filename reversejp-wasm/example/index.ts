import { file, serve, type ServeOptions } from "bun";
import { build } from "bun";
import { join } from "path";

// Build the TypeScript app on startup
console.log("📦 Building TypeScript application...");
await build({
  entrypoints: ["./src/app.ts"],
  outdir: "./public",
  target: "browser",
  minify: false,
  sourcemap: "external",
});
console.log("✅ Build complete!");

const PORT = process.env.PORT ? parseInt(process.env.PORT) : 3001;

const server = serve({
  port: PORT,
  async fetch(req: Request): Promise<Response> {
    const url = new URL(req.url);
    
    // Serve index.html
    if (url.pathname === "/") {
      return new Response(file("./public/index.html"), {
        headers: { "Content-Type": "text/html; charset=utf-8" },
      });
    }
    
    // Serve app.js (compiled from TypeScript)
    if (url.pathname === "/app.js") {
      return new Response(file("./public/app.js"), {
        headers: { 
          "Content-Type": "application/javascript; charset=utf-8",
          "Cross-Origin-Embedder-Policy": "require-corp",
          "Cross-Origin-Opener-Policy": "same-origin"
        },
      });
    }
    
    // Serve source maps
    if (url.pathname === "/app.js.map") {
      return new Response(file("./public/app.js.map"), {
        headers: { "Content-Type": "application/json; charset=utf-8" },
      });
    }
    
    // Serve styles.css
    if (url.pathname === "/styles.css") {
      return new Response(file("./public/styles.css"), {
        headers: { "Content-Type": "text/css; charset=utf-8" },
      });
    }
    
    // Serve any other static files from public directory
    const filePath = join("./public", url.pathname);
    try {
      const staticFile = file(filePath);
      if (await staticFile.exists()) {
        const ext = url.pathname.split('.').pop();
        const contentTypes: Record<string, string> = {
          'wasm': 'application/wasm',
          'js': 'application/javascript',
          'json': 'application/json',
          'css': 'text/css',
          'html': 'text/html',
          'png': 'image/png',
          'jpg': 'image/jpeg',
          'jpeg': 'image/jpeg',
          'svg': 'image/svg+xml',
        };
        return new Response(staticFile, {
          headers: { 
            "Content-Type": contentTypes[ext || ''] || "application/octet-stream",
            "Cross-Origin-Embedder-Policy": "require-corp",
            "Cross-Origin-Opener-Policy": "same-origin"
          },
        });
      }
    } catch (e) {
      // File doesn't exist, continue to 404
    }
    
    return new Response("Not Found", { status: 404 });
  },
});

console.log(`🚀 Server running at http://localhost:${server.port}`);
console.log(`📍 Open your browser to view the interactive map!`);
