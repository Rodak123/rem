import { useState } from 'react';
import { Tab, TABS } from '../types/tabs';
import { TabContext } from '../contexts/tabContext';

interface TabContextProviderProps {
    children: React.ReactNode;
    defaultTab?: Tab;
}

export const TabContextProvider: React.FC<TabContextProviderProps> = ({ children, defaultTab }) => {
    const [tabHistory, setTabHistory] = useState<Tab[]>([]);
    const [currentTab, setCurrentTab] = useState<Tab>(defaultTab ?? { type: TABS.HOME });

    const changeTo = (targetTab: Tab) => {
        setTabHistory([...tabHistory, currentTab]);
        setCurrentTab(targetTab);
    };

    const changeToPrevious = () => {
        const newTabHistory = [...tabHistory];
        const previousTab = newTabHistory.pop();
        setTabHistory(newTabHistory);
        setCurrentTab(previousTab ?? { type: TABS.HOME });
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