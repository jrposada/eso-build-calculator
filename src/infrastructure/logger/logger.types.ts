export type LogLevel = 'log' | 'info' | 'warn' | 'error' | 'success' | 'dim';

export interface Logger {
  log: (message: string, ...args: unknown[]) => void;
  info: (message: string, ...args: unknown[]) => void;
  warn: (message: string, ...args: unknown[]) => void;
  error: (message: string, ...args: unknown[]) => void;
  success: (message: string, ...args: unknown[]) => void;
  dim: (message: string, ...args: unknown[]) => void;
}
