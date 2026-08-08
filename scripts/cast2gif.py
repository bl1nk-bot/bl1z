#!/usr/bin/env python3
"""scripts/cast2gif.py - Render an asciinema .cast recording to an animated GIF.

Real screen recording -> real GIF: frames come from the actual cast output,
not hand-drawn text. Minimal ANSI subset (SGR colors, cursor moves, line
erases) — enough for CLI demo recordings.

Usage: python3 scripts/cast2gif.py demo.cast demo.gif [--fps 8]
"""
import argparse
import json
import re
import sys
from PIL import Image, ImageDraw, ImageFont

FONT = "/data/data/com.termux/files/usr/share/fonts/TTF/DejaVuSansMono.ttf"


def load_font(path, size=18):
    """Load truetype font, fall back to default if missing."""
    try:
        return ImageFont.truetype(path, size)
    except (OSError, IOError):
        return ImageFont.load_default()

PALETTE = {
    30: (0x00, 0x00, 0x00), 31: (0xCD, 0x00, 0x00), 32: (0x00, 0xCD, 0x00),
    33: (0xCD, 0xCD, 0x00), 34: (0x00, 0x00, 0xEE), 35: (0xCD, 0x00, 0xCD),
    36: (0x00, 0xCD, 0xCD), 37: (0xE5, 0xE5, 0xE5),
    90: (0x7F, 0x7F, 0x7F), 91: (0xFF, 0x00, 0x00), 92: (0x00, 0xFF, 0x00),
    93: (0xFF, 0xFF, 0x00), 94: (0x5C, 0x5C, 0xFF), 95: (0xFF, 0x00, 0xFF),
    96: (0x00, 0xFF, 0xFF), 97: (0xFF, 0xFF, 0xFF),
}
BG = (0x0C, 0x0C, 0x0C)
FG = (0xE5, 0xE5, 0xE5)

SGR = re.compile(r"\x1b\[([0-9;]*)m")
CSI = re.compile(r"\x1b\[([0-9;]*)([A-Za-z])")
OTHER = re.compile(r"\x1b[^A-Za-z]*[A-Za-z]|[\x00-\x08\x0e-\x1f]")


class Cell:
    __slots__ = ("ch", "fg", "bg", "bold")

    def __init__(self, ch=" ", fg=FG, bg=BG, bold=False):
        self.ch, self.fg, self.bg, self.bold = ch, fg, bg, bold

    def same(self, o):
        return (self.ch, self.fg, self.bg, self.bold) == (o.ch, o.fg, o.bg, o.bold)


class Screen:
    def __init__(self, cols, rows):
        self.cols, self.rows = cols, rows
        self.grid = [[Cell() for _ in range(cols)] for _ in range(rows)]
        self.r = self.c = 0
        self.fg, self.bg, self.bold = FG, BG, False

    def put(self, ch):
        if self.r >= self.rows:
            self.r = self.rows - 1
            self.scroll()
        if ch == "\r":
            self.c = 0
            return
        if ch == "\n":
            self.r += 1
            if self.r >= self.rows:
                self.scroll()
            return
        if ch == "\b" and self.c > 0:
            self.c -= 1
            return
        if self.c >= self.cols:
            self.c = 0
            self.r += 1
            if self.r >= self.rows:
                self.scroll()
        self.grid[self.r][self.c] = Cell(ch, self.fg, self.bg, self.bold)
        self.c += 1

    def scroll(self):
        self.grid.pop(0)
        self.grid.append([Cell() for _ in range(self.cols)])
        self.r = self.rows - 1

    def clear_line(self):
        for c in range(self.c, self.cols):
            self.grid[self.r][c] = Cell()

    def sgr(self, args):
        if not args:
            self.fg, self.bg, self.bold = FG, BG, False
            return
        for a in args.split(";"):
            if not a:
                continue
            n = int(a)
            if n == 0:
                self.fg, self.bg, self.bold = FG, BG, False
            elif n == 1:
                self.bold = True
            elif 30 <= n <= 37 or 90 <= n <= 97:
                self.fg = PALETTE[n]
            elif 40 <= n <= 47 or 100 <= n <= 107:
                self.bg = PALETTE[n - 10]  # 40-47 -> 30-37, 100-107 -> 90-97
            # 39/49 default colors, 38;5;N 256-color: ignored (ponytail: demo
            # output only uses the base palette)

    def feed(self, data):
        i = 0
        while i < len(data):
            ch = data[i]
            if ch == "\x1b":
                m = SGR.match(data, i)
                if m:
                    self.sgr(m.group(1))
                    i = m.end()
                    continue
                m = CSI.match(data, i)
                if m:
                    args, fn = m.group(1), m.group(2)
                    n = int(args) if args else 1
                    if fn == "A":
                        self.r = max(0, self.r - n)
                    elif fn == "B":
                        self.r = min(self.rows - 1, self.r + n)
                    elif fn == "C":
                        self.c = min(self.cols - 1, self.c + n)
                    elif fn == "D":
                        self.c = max(0, self.c - n)
                    elif fn == "K":
                        self.clear_line()
                    elif fn == "J":
                        if n == 2:
                            self.grid = [[Cell() for _ in range(self.cols)]
                                         for _ in range(self.rows)]
                            self.r = self.c = 0
                    i = m.end()
                    continue
                m = OTHER.match(data, i)
                if m:
                    i = m.end()
                    continue
                i += 1
            else:
                self.put(ch)
                i += 1

    def snapshot(self):
        return (
            tuple(
                tuple((cell.ch, cell.fg, cell.bg, cell.bold) for cell in row)
                for row in self.grid
            ),
            self.r,
            self.c,
        )


def render(screen, font, cell_w, cell_h, cursor=True):
    img = Image.new("RGB", (screen.cols * cell_w, screen.rows * cell_h), BG)
    d = ImageDraw.Draw(img)
    for r, row in enumerate(screen.grid):
        for c, cell in enumerate(row):
            if cell.bg != BG:
                d.rectangle(
                    [c * cell_w, r * cell_h, (c + 1) * cell_w - 1,
                     (r + 1) * cell_h - 1], fill=cell.bg)
            if cell.ch != " ":
                color = cell.fg if not cell.bold else tuple(
                    min(255, int(v * 1.4)) for v in cell.fg)
                d.text((c * cell_w, r * cell_h), cell.ch, font=font,
                       fill=color)
    if cursor:
        r, c = screen.r, screen.c
        if r < screen.rows:
            d.rectangle([c * cell_w, r * cell_h, (c + 1) * cell_w - 1,
                         (r + 1) * cell_h - 1], outline=FG, width=1)
    return img


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cast")
    ap.add_argument("out")
    ap.add_argument("--fps", type=int, default=8)
    ap.add_argument("--font", default=FONT, help="Path to TTF font file")
    args = ap.parse_args()
    if args.fps < 1:
        ap.error("--fps must be greater than zero")

    events = []
    with open(args.cast, encoding="utf-8", errors="replace") as f:
        header = json.loads(f.readline())
        for line in f:
            t, kind, data = json.loads(line)
            if kind == "o":
                events.append((t, data))
    cols, rows = header["width"], header["height"]

    font = load_font(args.font, 18)
    cell_w = int(font.getlength("M")) + 1
    cell_h = 22
    screen = Screen(cols, rows)

    frames, last_sig = [], None
    last_t, next_t = 0.0, 1.0 / args.fps
    for t, data in events:
        # Capture samples BEFORE feeding new data (render pre-event state)
        while next_t <= t + 1e-9:
            sig = screen.snapshot()
            if sig != last_sig:
                last_sig = sig
                frames.append((next_t, render(screen, font, cell_w, cell_h)))
            next_t += 1.0 / args.fps
        screen.feed(data)
    if not frames:
        frames.append((0.0, render(screen, font, cell_w, cell_h)))

    # GIF: delay in centiseconds, keep each static frame on screen a while
    imgs, delays = [], []
    for i, (t, img) in enumerate(frames):
        imgs.append(img)
        nxt = frames[i + 1][0] if i + 1 < len(frames) else t + 1.0
        delays.append(max(5, int(round((nxt - t) * 100))))
    imgs[0].save(args.out, save_all=True, append_images=imgs[1:],
                 duration=delays, loop=0, optimize=False)
    print(f"wrote {args.out}: {len(frames)} frames, "
          f"{cols}x{rows} cols, {imgs[0].size[0]}x{imgs[0].size[1]}px")


if __name__ == "__main__":
    sys.exit(main())
