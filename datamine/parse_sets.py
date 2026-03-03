"""Parse UESP ESO Sets HTML table into JSON."""

from __future__ import annotations

import json
import re
from html.parser import HTMLParser
from pathlib import Path


class TableParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.rows = []
        self._current_row = []
        self._current_cell = ""
        self._in_table = False
        self._in_row = False
        self._in_cell = False
        self._is_header = False
        self._skip_header = True

    def handle_starttag(self, tag, attrs):
        if tag == "table":
            self._in_table = True
        elif tag == "tr" and self._in_table:
            self._in_row = True
            self._current_row = []
        elif tag in ("td", "th") and self._in_row:
            self._in_cell = True
            self._is_header = tag == "th"
            self._current_cell = ""
        elif tag == "br" and self._in_cell:
            self._current_cell += "\n"

    def handle_endtag(self, tag):
        if tag == "table":
            self._in_table = False
        elif tag == "tr" and self._in_row:
            self._in_row = False
            if self._skip_header:
                self._skip_header = False
            elif self._current_row:
                self.rows.append(self._current_row)
        elif tag in ("td", "th") and self._in_cell:
            self._in_cell = False
            if not self._is_header:
                self._current_row.append(self._current_cell.strip())

    def handle_data(self, data):
        if self._in_cell:
            self._current_cell += data


def parse_bonus_line(line: str) -> dict | None:
    """Parse a single bonus line like '(2 items) Adds 25-1096 Maximum Stamina'."""
    m = re.match(r"^\((\d+)\s+items?\)\s+(.*)", line.strip())
    if not m:
        return None

    piece_count = int(m.group(1))
    description = m.group(2).strip()

    bonus = {
        "pieces": piece_count,
        "description": description,
    }

    # Try to parse stat bonuses like "Adds 25-1096 Maximum Stamina"
    stat_match = re.match(r"Adds\s+(\d+)-(\d+)\s+(.+)", description)
    if stat_match:
        bonus["type"] = "stat"
        bonus["value_min"] = int(stat_match.group(1))
        bonus["value_max"] = int(stat_match.group(2))
        bonus["stat"] = stat_match.group(3).strip()
        return bonus

    # Try to parse percentage bonuses like "Adds 4% Healing Taken"
    pct_match = re.match(r"Adds\s+(\d+(?:\.\d+)?)%\s+(.+)", description)
    if pct_match:
        bonus["type"] = "stat_percent"
        bonus["value"] = float(pct_match.group(1))
        bonus["stat"] = pct_match.group(2).strip()
        return bonus

    # Try "Gain <buff> at all times, <effect>" pattern
    gain_match = re.match(r"Gain\s+(.+?)\s+at all times,?\s*(.*)", description)
    if gain_match:
        bonus["type"] = "permanent_buff"
        bonus["buff"] = gain_match.group(1).strip()
        if gain_match.group(2):
            bonus["effect"] = gain_match.group(2).strip().rstrip(".")
        return bonus

    # Everything else is a proc/special effect
    bonus["type"] = "effect"
    return bonus


def split_perfected_bonus(line: str) -> list[str]:
    """Split lines that contain a perfected bonus marker mid-line.

    E.g. '(4 items) Adds 34-1487 Offensive Penetration (5 perfected items) Adds 15-657 Critical Chance'
    becomes two entries.
    """
    m = re.match(
        r"^(\(\d+\s+items?\)\s+Adds\s+\d+-\d+\s+\S+(?:\s+\S+)*?)"
        r"\s+\((\d+)\s+perfected\s+items?\)\s+(.*)",
        line,
    )
    if m:
        base_line = m.group(1)
        perf_count = m.group(2)
        perf_desc = m.group(3)
        return [base_line, f"({perf_count} items) {perf_desc}"]
    return [line]


def parse_bonuses(bonuses_text: str) -> list[dict]:
    """Parse the full bonuses cell into a list of bonus entries."""
    bonuses = []

    # Split on bonus markers, but keep multiline descriptions together
    lines = bonuses_text.split("\n")
    current_line = ""

    for line in lines:
        line = line.strip()
        if not line:
            continue

        # Check if this line starts a new bonus entry
        if re.match(r"^\(\d+\s+items?\)", line):
            # Flush previous
            if current_line:
                for part in split_perfected_bonus(current_line):
                    bonus = parse_bonus_line(part)
                    if bonus:
                        bonuses.append(bonus)
            current_line = line
        else:
            # Continuation of previous bonus description
            if current_line:
                current_line += " " + line
            else:
                current_line = line

    # Flush last
    if current_line:
        for part in split_perfected_bonus(current_line):
            bonus = parse_bonus_line(part)
            if bonus:
                bonuses.append(bonus)

    return bonuses


def parse_item_slots(slots_text: str) -> list[str]:
    """Parse item slots string into a list of slot entries."""
    if not slots_text.strip():
        return []
    # Split on whitespace but keep parenthesized groups like "Weapons(Lightning Frost Flame)"
    return re.findall(r'\w+\([^)]*\)|\w+', slots_text)


def main():
    base = Path(__file__).parent
    input_path = base / "raw" / "UESP_ESO Sets.html"
    output_path = base / "parsed" / "sets.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)

    html = input_path.read_text(encoding="utf-8")

    parser = TableParser()
    parser.feed(html)

    # Columns: Set Name, Bonuses, Type, Category, Sources, Item Slots, Image
    sets = []
    for row in parser.rows:
        if len(row) < 7:
            continue

        name, bonuses_raw, set_type, category, sources, item_slots, _image = row[:7]

        bonuses = parse_bonuses(bonuses_raw)

        entry = {
            "name": name.strip(),
            "bonuses": bonuses,
            "type": set_type.strip() if set_type.strip() else None,
            "category": category.strip() if category.strip() else None,
            "sources": sources.strip() if sources.strip() else None,
            "item_slots": parse_item_slots(item_slots),
        }
        sets.append(entry)

    output_path.write_text(
        json.dumps(sets, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Parsed {len(sets)} sets -> {output_path}")


if __name__ == "__main__":
    main()
