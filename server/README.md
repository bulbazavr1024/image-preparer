# Image Preparer Server

HTTP API server for image/video compression, format conversion, and metadata inspection.

## Quick Start

```bash
# From workspace root
cargo run --release --bin server

# Or from server directory
cargo run --release
```

The server will start on `http://0.0.0.0:3000`

## API Endpoints

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

**Example:**
```bash
curl http://localhost:3000/health
```

---

### POST /compress

Compress images or videos.

**Form Fields:**
- `file` (required): Binary file data
- `quality` (optional): 0-100, default 80
- `speed` (optional): 1-10, default 3
- `no_lossy` (optional): "true" or "false", default false
- `strip` (optional): "all", "safe", or "none", default "all"

**Response:**
- Binary file data (compressed)

**Example:**
```bash
# Compress PNG with quality 85
curl -X POST \
  -F "file=@input.png" \
  -F "quality=85" \
  -F "strip=all" \
  -o compressed.png \
  http://localhost:3000/compress

# Compress MP4 video
curl -X POST \
  -F "file=@video.mp4" \
  -F "quality=75" \
  -F "speed=5" \
  -o compressed.mp4 \
  http://localhost:3000/compress

# Lossless compression
curl -X POST \
  -F "file=@image.webp" \
  -F "no_lossy=true" \
  -o compressed.webp \
  http://localhost:3000/compress
```

---

### POST /convert

Convert between image formats (PNG, JPG, WebP).

**Form Fields:**
- `file` (required): Binary file data
- `to` (required): Target format - "png", "jpg", "jpeg", or "webp"
- `quality` (optional): 0-100, default 80
- `no_lossy` (optional): "true" or "false", default false

**Response:**
- Binary file data (converted)

**Example:**
```bash
# Convert PNG to JPG
curl -X POST \
  -F "file=@image.png" \
  -F "to=jpg" \
  -F "quality=90" \
  -o output.jpg \
  http://localhost:3000/convert

# Convert JPG to WebP
curl -X POST \
  -F "file=@photo.jpg" \
  -F "to=webp" \
  -F "quality=80" \
  -o output.webp \
  http://localhost:3000/convert

# Lossless WebP conversion
curl -X POST \
  -F "file=@image.png" \
  -F "to=webp" \
  -F "no_lossy=true" \
  -o output.webp \
  http://localhost:3000/convert
```

---

### POST /inspect

View file metadata.

**Form Fields:**
- `file` (required): Binary file data

**Response:**
```json
{
  "success": true,
  "data": {
    "format": "png",
    "size": 123456,
    "metadata": {
      "note": "Detailed metadata extraction coming soon"
    }
  },
  "error": null
}
```

**Example:**
```bash
curl -X POST \
  -F "file=@image.png" \
  http://localhost:3000/inspect | jq .
```

---

### POST /extract

Extract frames from MP4 video (not yet implemented).

**Form Fields:**
- `file` (required): Binary MP4 file
- `fps` (optional): Frames per second, default 1, 0=all frames

**Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Frame extraction not yet implemented for web API"
}
```

**Note:** Frame extraction is currently only available via the CLI tool.

---

## Supported Formats

### Images
- **PNG**: Lossy + lossless compression, metadata stripping
- **WebP**: Lossy/lossless compression, metadata stripping
- **JPEG**: Format conversion only (no compression)

### Audio
- **MP3**: Metadata stripping only

### Video
- **MP4**: Compression (requires ffmpeg), metadata stripping

## Error Handling

On error, the API returns a JSON response:

```json
{
  "success": false,
  "data": null,
  "error": "Error message here"
}
```

Common HTTP status codes:
- `200 OK`: Success
- `400 BAD_REQUEST`: Missing or invalid parameters
- `415 UNSUPPORTED_MEDIA_TYPE`: Unsupported file format
- `500 INTERNAL_SERVER_ERROR`: Processing error

## Configuration

The server runs on port 3000 by default. To change the port, modify `server/src/main.rs`:

```rust
let addr = "0.0.0.0:8080"; // Change port here
```

## CORS

CORS is enabled with permissive settings for all origins. This is suitable for development but should be restricted in production.

## System Requirements

- **Rust** 1.70+
- **ffmpeg** (required for MP4 processing)
  - macOS: `brew install ffmpeg`
  - Linux: `apt install ffmpeg`

## Architecture

The server reuses the CLI library (`image_preparer`) for all processing logic:
- `src/main.rs`: Server setup and route definitions
- `src/handlers.rs`: API endpoint handlers
- CLI library: Processing pipeline, format processors, converters

## Development

```bash
# Run in debug mode
cargo run --bin server

# Run in release mode
cargo run --release --bin server

# Build only
cargo build --release --bin server

# Run with verbose logging
RUST_LOG=debug cargo run --bin server
```

## Testing with curl

```bash
# Health check
curl http://localhost:3000/health

# Compress image
curl -X POST \
  -F "file=@test.png" \
  -F "quality=80" \
  -o output.png \
  http://localhost:3000/compress

# Convert format
curl -X POST \
  -F "file=@test.png" \
  -F "to=webp" \
  -o output.webp \
  http://localhost:3000/convert

# Inspect metadata
curl -X POST \
  -F "file=@test.png" \
  http://localhost:3000/inspect | jq .
```

## Production Deployment

For production use, consider:
1. Configure CORS for specific origins
2. Add authentication/authorization
3. Add rate limiting
4. Use a reverse proxy (nginx, Caddy)
5. Enable HTTPS
6. Set up logging and monitoring
7. Handle file size limits
8. Add input validation and sanitization

## License

GPL-3.0-or-later

## Related

- [CLI Documentation](../cli/README.md)
- [Installation Guide](../cli/INSTALL.md)
- [Development Guide](../cli/CLAUDE.md)
