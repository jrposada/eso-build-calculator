# Datamine

Data sources and scripts for extracting ESO game data.

## Sources

- [UESP Set Reference](https://esoitem.uesp.net/setReference.php) - Gear set bonuses
- [UESP Skill Coefficients](https://esoitem.uesp.net/viewSkillCoef.php) - Skill damage coefficients

## Scripts

- `extract_sets.py` - Extracts gear set data from the UESP HTML into `sets.json`
- `generate_sets_rs.py` - Generates Rust source files from `sets.json` into `src/data/sets/`

## Usage

```bash
# Extract sets from saved HTML page
python3 datamine/extract_sets.py

# Generate Rust set data files
python3 datamine/generate_sets_rs.py
```
