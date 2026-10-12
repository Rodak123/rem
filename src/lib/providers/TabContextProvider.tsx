import { useState } from 'react';
import { Tab } from '../types/tabs';
import { TabContext } from '../contexts/tabContext';

interface TabContextProviderProps {
    children: React.ReactNode;
    defaultTab?: Tab;
}

// Allows components to manage tabs
export const TabContextProvider: React.FC<TabContextProviderProps> = ({ children, defaultTab }) => {
    const [tabHistory, setTabHistory] = useState<Tab[]>([]);
    const [currentTab, setCurrentTab] = useState<Tab>(defaultTab ?? { type: "Home" });

    const changeTo = (targetTab: Tab) => {
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