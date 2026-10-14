import { clsx, type ClassValue } from 'clsx';

// Allows conditionals and random bullsh*t go in the className attribute.

/**
 * Evaluates almost any values into a useable className attribute.
 */
export const cn = (...inputs: ClassValue[]) => clsx(inputs);