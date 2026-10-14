import { useState } from 'react';
import { Tab } from '../types/tabs';
import { TabContext } from '../contexts/tabContext';

interface TabContextProviderProps {
    children: React.ReactNode;
    defaultTab?: Tab;
}

/**
 * Stores tab state and history, and exposes methods to manage manipulate with that state.
 */
export const TabContextProvider: React.FC<TabContextProviderProps> = ({ children, defaultTab }) => {
    const [tabHistory, setTabHistory] = useState<Tab[]>([]);
    const [currentTab, setCurrentTab] = useState<Tab>(defaultTab ?? { type: "Home" });

    const changeTo = (targetTab: Tab) => {
        // There is no check here for reopening the same tab,
        // so make sure that UI never allows it.

        // If we want that, here's a simple solution:
        // if (JSON.stringify(currentTab) === JSON.stringify(targetTab)) return;

        setTabHistory([...tabHistory, currentTab]);
        setCurrentTab(targetTab);
    };

    const changeToPrevious = () => {
        const newTabHistory = [...tabHistory];
        const previousTab = newTabHistory.pop();
        setTabHistory(newTabHistory);
        setCurrentTab(previousTab ?? { type: "Home" });
    };

    return (
        <TabContext.Provider value={{
            currentTab,
            tabHistory,
            previousTab: tabHistory[tabHistory.length - 1] ?? null,
            changeTo,
            changeToPrevious
        }}>
            {children}
        </TabContext.Provider>
    );
};