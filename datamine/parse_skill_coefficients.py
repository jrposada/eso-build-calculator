"""Parse UESP ESO Skill Coefficients HTML table into JSON."""

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


def parse_equation(eq_text: str) -> dict:
    """Parse a single equation like '<<1>> = 0.09685 MaxStat + 1.0169 MaxPower  (Ultimate, ...)'."""
    # Match the <<N>> = ... pattern
    m = re.match(r"<<(\d+)>>\s*=\s*(.*)", eq_text.strip())
    if not m:
        return None

    index = int(m.group(1))
    rhs = m.group(2).strip()

    # Check if it's a constant value
    # Constants look like: "420 (Constant)", "3% (Constant)", "2 hours (Constant)",
    # "Major Mending (Constant)", "Your Companion (Constant)", "6 seconds (Constant)"
    const_match = re.match(r"^(.*?)\s*\(Constant\)\s*$", rhs)
    if const_match:
        return {
            "index": index,
            "type": "constant",
            "value": const_match.group(1).strip(),
        }

    # It's a coefficient equation. Parse the formula and metadata.
    # Examples:
    #   "0.09685 MaxStat + 1.0169 MaxPower  (Ultimate, ratio = 10.50, Dmg, Frost, AOE, DOT, ...)"
    #   "+ 0.25 WD  (Stamina, Dmg, Magic, SingleTarget, DOT, 3s duration, ...)"
    #   "0.3099 Health  (Health, DmgShield, SingleTarget, Direct, 6s duration, ...)"
    #   "0.044697 MaxStat + 0.46932 MaxPower  (Ultimate, ratio = 10.50, Heal, AOE, ...)"
    #   "+ 3 WD  (Stamina, Dmg, Magic, SingleTarget, Direct, ...)"

    # Split formula from metadata parentheses (find the last top-level parenthetical)
    paren_match = re.search(r"\(([^()]+)\)\s*$", rhs)
    if paren_match:
        formula = rhs[: paren_match.start()].strip()
        meta_str = paren_match.group(1).strip()
    else:
        formula = rhs
        meta_str = ""

    result = {
        "index": index,
        "type": "scaling",
        "formula": formula,
        "terms": [],
    }

    # Parse formula terms
    # Patterns: "0.09685 MaxStat", "1.0169 MaxPower", "0.25 WD", "3 WD", "0.3099 Health"
    # Can also have a leading "+" or be combined with "+"
    # Normalize: remove leading +
    formula_clean = formula.lstrip("+ ").strip()
    # Split on " + " to get individual terms
    term_parts = re.split(r"\s*\+\s*", formula_clean)
    for part in term_parts:
        part = part.strip()
        if not part:
            continue
        # Match: coefficient stat_name (e.g. "0.09685 MaxStat", "1.0169 MaxPower", "3 WD")
        tm = re.match(r"^(-?[\d.]+)\s+(.+)$", part)
        if tm:
            result["terms"].append(
                {"coefficient": float(tm.group(1)), "stat": tm.group(2).strip()}
            )
        else:
            # Fallback: store raw
            result["terms"].append({"raw": part})

    # Parse metadata tags
    if meta_str:
        meta = parse_metadata(meta_str)
        result["metadata"] = meta

    return result


def parse_metadata(meta_str: str) -> dict:
    """Parse the parenthetical metadata string into structured data."""
    meta = {"tags": []}

    parts = [p.strip() for p in meta_str.split(",")]
    for part in parts:
        if not part:
            continue
        # Key-value pairs like "ratio = 10.50", "R2 = 1"
        kv = re.match(r"^(\w[\w\s]*?)\s*=\s*(.+)$", part)
        if kv:
            key = kv.group(1).strip()
            val = kv.group(2).strip()
            # Try numeric
            try:
                val = float(val)
                if val == int(val):
                    val = int(val)
            except ValueError:
                pass
            meta[key] = val
        # Duration-like: "3s duration", "-0.001s duration", "1s tick", "0.3s cooldown"
        elif re.match(r"^-?[\d.]+s\s+\w+", part):
            dm = re.match(r"^(-?[\d.]+)s\s+(\w+)", part)
            if dm:
                val = float(dm.group(1))
                key = dm.group(2)
                meta[key] = val
        else:
            meta["tags"].append(part)

    # Clean up empty tags
    if not meta["tags"]:
        del meta["tags"]

    return meta


def main():
    base = Path(__file__).parent
    input_path = base / "raw" / "UESP_ESO Skill Coefficients.html"
    output_path = base / "parsed" / "skill_coefficients.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)

    html = input_path.read_text(encoding="utf-8")

    parser = TableParser()
    parser.feed(html)

    # Columns: Skill Name, ID, Mechanic, Class, Skill Line, #, Description, Equations
    skills = []
    for row in parser.rows:
        if len(row) < 8:
            continue

        name, ability_id, mechanic, cls, skill_line, eq_count, description, equations_raw = row

        # Parse equations
        equations = []
        if equations_raw:
            for line in equations_raw.split("\n"):
                line = line.strip()
                if not line:
                    continue
                eq = parse_equation(line)
                if eq:
                    equations.append(eq)

        skill = {
            "name": name.strip(),
            "id": ability_id.strip(),
            "mechanic": mechanic.strip(),
            "class": cls.strip() if cls.strip() else None,
            "skill_line": skill_line.strip() if skill_line.strip() else None,
            "equation_count": int(eq_count) if eq_count.strip().isdigit() else 0,
            "description": description.strip(),
            "equations": equations,
        }
        skills.append(skill)

    output_path.write_text(
        json.dumps(skills, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Parsed {len(skills)} skills -> {output_path}")


if __name__ == "__main__":
    main()
