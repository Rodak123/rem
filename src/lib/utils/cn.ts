import { clsx, type ClassValue } from 'clsx';

// Allows conditionals and random bullsh*t go in the className attribute.
export const cn = (...inputs: ClassValue[]) => clsx(inputs);