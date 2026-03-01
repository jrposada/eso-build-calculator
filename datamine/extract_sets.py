#!/usr/bin/env python3
"""Extract ESO gear set data from UESP HTML table into structured JSON."""

import json
import re
import sys
from datetime import datetime, timezone
from html.parser import HTMLParser
from pathlib import Path

# ---------------------------------------------------------------------------
# HTML table parser
# ---------------------------------------------------------------------------

class SetTableParser(HTMLParser):
    """Parse the UESP set reference HTML table into raw row data."""

    def __init__(self):
        super().__init__()
        self.sets: list[dict] = []
        self._in_tbody = False
        self._in_row = False
        self._in_cell = False
        self._in_bold = False
        self._col_idx = 0
        self._cell_text = ""
        self._current_row: list[str] = []

    def handle_starttag(self, tag, attrs):
        if tag == "tbody":
            self._in_tbody = True
        elif tag == "tr" and self._in_tbody:
            self._in_row = True
            self._col_idx = 0
            self._current_row = []
        elif tag == "td" and self._in_row:
            self._in_cell = True
            self._cell_text = ""
        elif tag == "b" and self._in_cell:
            self._in_bold = True

    def handle_endtag(self, tag):
        if tag == "tbody":
            self._in_tbody = False
        elif tag == "tr" and self._in_row:
            self._in_row = False
            if len(self._current_row) >= 6:
                self.sets.append({
                    "name": self._current_row[0],
                    "bonuses_raw": self._current_row[1],
                    "html_type": self._current_row[2].strip(),
                    "category": self._current_row[3].strip(),
                    "sources": self._current_row[4].strip(),
                    "item_slots": self._current_row[5].strip(),
                })
        elif tag == "td" and self._in_cell:
            self._in_cell = False
            self._current_row.append(self._cell_text)
            self._col_idx += 1
        elif tag == "b" and self._in_bold:
            self._in_bold = False

    def handle_data(self, data):
        if self._in_cell:
            # For the image column (col 6), we skip most data via the col
            # count check in handle_endtag, but we still accumulate text.
            self._cell_text += data


# ---------------------------------------------------------------------------
# Bonus parsing
# ---------------------------------------------------------------------------

# Matches "(N item)" or "(N items)"
PIECE_RE = re.compile(r"\((\d+)\s+items?\)\s*")

# "Adds X-Y StatName" (stat bonus with min-max range)
STAT_RE = re.compile(r"^Adds\s+(\d+)-(\d+)\s+(.+)$")

# "Adds N% StatName" (percent bonus)
PERCENT_RE = re.compile(r"^Adds\s+(\d+)%\s+(.+)$")


def parse_bonuses(raw: str) -> list[dict]:
    """Split raw bonus text into individual bonus entries."""
    bonuses: list[dict] = []

    # Split on "(N items)" markers. We use finditer to locate all markers,
    # then slice the text between them.
    markers = list(PIECE_RE.finditer(raw))
    if not markers:
        return bonuses

    for i, m in enumerate(markers):
        piece_count = int(m.group(1))
        start = m.end()
        end = markers[i + 1].start() if i + 1 < len(markers) else len(raw)
        text = raw[start:end].strip()
        if not text:
            continue

        bonus: dict = {
            "piece_count": piece_count,
            "raw_text": text,
        }

        stat_m = STAT_RE.match(text)
        pct_m = PERCENT_RE.match(text)

        if stat_m:
            bonus["bonus_type"] = "stat"
            bonus["stat_name"] = stat_m.group(3)
            bonus["value"] = int(stat_m.group(2))
        elif pct_m:
            bonus["bonus_type"] = "percent_stat"
            bonus["stat_name"] = pct_m.group(2)
            bonus["value"] = int(pct_m.group(1))
        else:
            bonus["bonus_type"] = "complex"

        bonuses.append(bonus)

    return bonuses


# ---------------------------------------------------------------------------
# Type mapping
# ---------------------------------------------------------------------------

TYPE_MAP = {
    "Monster": "Monster",
    "Mythic": "Mythic",
    "Arena": "Arena",
}


def map_set_type(html_type: str) -> str:
    return TYPE_MAP.get(html_type, "Normal")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    here = Path(__file__).parent
    html_path = here / "UESP_ESO Log Data -- Set Reference.html"
    out_path = here / "sets.json"

    if not html_path.exists():
        print(f"ERROR: {html_path} not found", file=sys.stderr)
        sys.exit(1)

    html = html_path.read_text(encoding="utf-8")

    parser = SetTableParser()
    parser.feed(html)

    sets = []
    type_counts: dict[str, int] = {}
    bonus_type_counts = {"stat": 0, "percent_stat": 0, "complex": 0}

    for raw_set in parser.sets:
        bonuses = parse_bonuses(raw_set["bonuses_raw"])
        set_type = map_set_type(raw_set["html_type"])

        type_counts[set_type] = type_counts.get(set_type, 0) + 1
        for b in bonuses:
            bonus_type_counts[b["bonus_type"]] = bonus_type_counts.get(b["bonus_type"], 0) + 1

        sets.append({
            "name": raw_set["name"],
            "html_type": raw_set["html_type"],
            "set_type": set_type,
            "sources": raw_set["sources"],
            "item_slots": raw_set["item_slots"],
            "bonuses": bonuses,
        })

    output = {
        "meta": {
            "total_sets": len(sets),
            "game_update": "48",
            "extracted_at": datetime.now(timezone.utc).isoformat(),
        },
        "sets": sets,
    }

    out_path.write_text(json.dumps(output, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    # Summary
    print(f"Extracted {len(sets)} sets -> {out_path}")
    print(f"\nSet types:")
    for t, c in sorted(type_counts.items(), key=lambda x: -x[1]):
        print(f"  {t}: {c}")
    print(f"\nBonus classifications:")
    for t, c in sorted(bonus_type_counts.items(), key=lambda x: -x[1]):
        print(f"  {t}: {c}")


if __name__ == "__main__":
    main()
