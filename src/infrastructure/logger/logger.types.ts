export type LogLevel =
  | 'log'
  | 'info'
  | 'warn'
  | 'error'
  | 'success'
  | 'dim'
  | 'progress';

export interface Logger {
  log: (message: string, ...args: unknown[]) => void;
  info: (message: string, ...args: unknown[]) => void;
  warn: (message: string, ...args: unknown[]) => void;
  error: (message: string, ...args: unknown[]) => void;
  success: (message: string, ...args: unknown[]) => void;
  dim: (message: string, ...args: unknown[]) => void;
  /** Overwrites the previous line if it was also a progress call */
  progress: (message: string) => void;
}
