interface ColumnDefinition {
  header: string;
  width: number;
  align?: 'left' | 'right'; // default: 'left'
}

interface TableOptions {
  title?: string;
  columns: ColumnDefinition[];
  dividerChar?: string; // default: '─'
  footer?: string;
}

function formatCell(
  value: string,
  width: number,
  align: 'left' | 'right',
): string {
  const truncated =
    value.length > width ? value.slice(0, width - 1) + '…' : value;
  return align === 'right'
    ? truncated.padStart(width)
    : truncated.padEnd(width);
}

export function table(data: string[][], options: TableOptions): string {
  const { title, columns, dividerChar = '─', footer } = options;

  // Calculate total width: sum of column widths + spacing between columns
  const totalWidth =
    columns.reduce((sum, col) => sum + col.width, 0) + columns.length - 1;
  const divider = dividerChar.repeat(totalWidth);

  const lines: string[] = [''];

  // Add title if provided
  if (title) {
    lines.push(title);
  }

  lines.push(divider);

  // Add header row
  const headerCells = columns.map((col) =>
    formatCell(col.header, col.width, col.align ?? 'left'),
  );
  lines.push(headerCells.join(' '));

  lines.push(divider);

  // Add data rows
  for (const row of data) {
    const cells = columns.map((col, i) => {
      const value = row[i] ?? '';
      return formatCell(value, col.width, col.align ?? 'left');
    });
    lines.push(cells.join(' '));
  }

  lines.push(divider);

  // Add footer if provided
  if (footer) {
    lines.push(footer);
  }

  lines.push('');

  return lines.join('\n');
}
