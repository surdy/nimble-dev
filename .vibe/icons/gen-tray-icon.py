#!/usr/bin/env python3
"""Generate monochrome tray icon PNGs for macOS template rendering."""
import struct, zlib, os

def create_tray_icon_png(width, height, filename):
    pixels = bytearray(width * height * 4)

    def set_pixel(x, y, r, g, b, a):
        if 0 <= x < width and 0 <= y < height:
            idx = (y * width + x) * 4
            pixels[idx] = r
            pixels[idx + 1] = g
            pixels[idx + 2] = b
            pixels[idx + 3] = a

    def fill_rect(x1, y1, w, h, r, g, b, a):
        for py in range(max(0, y1), min(y1 + h, height)):
            for px in range(max(0, x1), min(x1 + w, width)):
                set_pixel(px, py, r, g, b, a)

    def fill_circle(cx, cy, radius, r, g, b, a):
        ri = int(radius) + 1
        for py in range(max(0, int(cy) - ri), min(height, int(cy) + ri + 1)):
            for px in range(max(0, int(cx) - ri), min(width, int(cx) + ri + 1)):
                if (px - cx) ** 2 + (py - cy) ** 2 <= radius ** 2:
                    set_pixel(px, py, r, g, b, a)

    s = width / 44.0

    # Faint rounded-square background (alpha only — macOS template uses alpha as mask)
    bg_r = 9.5 * s
    for py in range(height):
        for px in range(width):
            def in_rounded_rect(px, py, left, top, w, h, r):
                cx = max(left + r, min(px, left + w - r))
                cy = max(top + r, min(py, top + h - r))
                return (px - cx) ** 2 + (py - cy) ** 2 <= r ** 2
            if in_rounded_rect(px, py, 0, 0, width, height, bg_r):
                set_pixel(px, py, 0, 0, 0, 64)

    # Card 3 (back) — medium alpha
    fill_rect(int(10 * s), int(27 * s), int(24 * s), int(5 * s), 0, 0, 0, 128)
    # Card 2 (middle) — higher alpha
    fill_rect(int(7 * s), int(21 * s), int(30 * s), int(6 * s), 0, 0, 0, 178)
    # Card 1 (front) — fully opaque, darkest
    fill_rect(int(4 * s), int(14 * s), int(36 * s), int(7 * s), 0, 0, 0, 255)

    def chunk(ctype, data):
        c = ctype + data
        crc = zlib.crc32(c) & 0xFFFFFFFF
        return struct.pack(">I", len(data)) + c + struct.pack(">I", crc)

    raw = b""
    for y in range(height):
        raw += b"\x00"
        raw += bytes(pixels[y * width * 4 : (y + 1) * width * 4])

    png = (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw))
        + chunk(b"IEND", b"")
    )

    os.makedirs(os.path.dirname(filename) or ".", exist_ok=True)
    with open(filename, "wb") as f:
        f.write(png)
    print(f"Created {filename} ({width}x{height})")


create_tray_icon_png(22, 22, "src-tauri/icons/tray-icon.png")
create_tray_icon_png(44, 44, "src-tauri/icons/tray-icon@2x.png")
