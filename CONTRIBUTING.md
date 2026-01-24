# Contributing

## Adding Skills

### Source Data

Skill data is extracted from [ESO Hub Skills](https://eso-hub.com/en/skills/). Each class has skill lines that can be found at URLs like:

- `https://eso-hub.com/en/skills/nightblade/shadow`
- `https://eso-hub.com/en/skills/nightblade/assassination`
- `https://eso-hub.com/en/skills/nightblade/siphoning`

Copy the skill descriptions from the relevant ESO Hub page and paste them into `RAW.md` in the repository root.

### Using the /add-skills Command

Once you have the raw skill data in `RAW.md`, use Claude Code's `/add-skills` command to generate the skill definitions:

```bash
/add-skills <class>/<skill-line>
```

Examples:

```bash
/add-skills nightblade/shadow
/add-skills nightblade/assassination
/add-skills nightblade/siphoning
```

The command will:

1. Read the raw skill data from `RAW.md`
2. Read the target skill file to understand the existing structure
3. Cross-reference to identify missing skills
4. Add the missing skills following the project conventions
5. Run TypeScript validation to ensure no errors
