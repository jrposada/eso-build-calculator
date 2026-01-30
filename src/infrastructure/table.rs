/// Column alignment
#[derive(Debug, Clone, Copy, Default)]
pub enum Align {
    #[default]
    Left,
    Right,
}

/// Column definition for table formatting
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub header: String,
    pub width: usize,
    pub align: Align,
}

impl ColumnDefinition {
    pub fn new(header: impl Into<String>, width: usize) -> Self {
        Self {
            header: header.into(),
            width,
            align: Align::Left,
        }
    }

    pub fn align_right(mut self) -> Self {
        self.align = Align::Right;
        self
    }

    pub fn align_left(mut self) -> Self {
        self.align = Align::Left;
        self
    }
}

/// Options for table formatting
#[derive(Debug, Clone, Default)]
pub struct TableOptions {
    pub title: Option<String>,
    pub columns: Vec<ColumnDefinition>,
    pub footer: Option<String>,
}

fn format_cell(value: &str, width: usize, align: Align) -> String {
    let truncated = if value.len() > width {
        format!("{}…", &value[..width.saturating_sub(1)])
    } else {
        value.to_string()
    };

    match align {
        Align::Right => format!("{:>width$}", truncated, width = width),
        Align::Left => format!("{:<width$}", truncated, width = width),
    }
}

/// Format data as an ASCII table
pub fn table(data: &[Vec<String>], options: TableOptions) -> String {
    let TableOptions {
        title,
        columns,
        footer,
    } = options;

    // Calculate total width: sum of column widths + spacing between columns
    let total_width: usize = columns.iter().map(|col| col.width).sum::<usize>() + columns.len() - 1;
    let divider = "-".repeat(total_width);

    let mut lines: Vec<String> = vec![String::new()];

    // Add title if provided
    if let Some(title) = title {
        lines.push(title);
    }

    lines.push(divider.clone());

    // Add header row
    let header_cells: Vec<String> = columns
        .iter()
        .map(|col| format_cell(&col.header, col.width, col.align))
        .collect();
    lines.push(header_cells.join(" "));

    lines.push(divider.clone());

    // Add data rows
    for row in data {
        let cells: Vec<String> = columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                let value = row.get(i).map(|s| s.as_str()).unwrap_or("");
                format_cell(value, col.width, col.align)
            })
            .collect();
        lines.push(cells.join(" "));
    }

    lines.push(divider);

    // Add footer if provided
    if let Some(footer) = footer {
        lines.push(footer);
    }

    lines.push(String::new());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cell_left() {
        assert_eq!(format_cell("test", 10, Align::Left), "test      ");
    }

    #[test]
    fn test_format_cell_right() {
        assert_eq!(format_cell("test", 10, Align::Right), "      test");
    }

    #[test]
    fn test_format_cell_truncate() {
        assert_eq!(format_cell("very long text", 5, Align::Left), "very…");
    }

    #[test]
    fn test_table_basic() {
        let data = vec![
            vec!["1".to_string(), "Alice".to_string()],
            vec!["2".to_string(), "Bob".to_string()],
        ];

        let output = table(
            &data,
            TableOptions {
                title: Some("Users".to_string()),
                columns: vec![
                    ColumnDefinition::new("ID", 4).align_right(),
                    ColumnDefinition::new("Name", 10),
                ],
                footer: None,
            },
        );

        assert!(output.contains("Users"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }
}
