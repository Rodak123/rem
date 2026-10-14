import { createContext } from 'react';
import { TabContextProps } from '../types/tabs';

export const TabContext = createContext<TabContextProps | null>(null);