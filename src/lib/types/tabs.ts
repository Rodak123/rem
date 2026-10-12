import { Album } from './audioTypes';

export type TabType = "Home" | "Album" | "Song" | "Queue" | "Lyrics" | "Settings" | "Conflicts";

interface TabBase {
    type: TabType;
}

export interface HomeTab extends TabBase {
    type: "Home";
}

export interface ALbumTab extends TabBase {
    type: "Album";
    album: Album,
}

export interface SongTab extends TabBase {
    type: "Song";
}

export interface QueueTab extends TabBase {
    type: "Queue";
}

export interface LyricsTab extends TabBase {
    type: "Lyrics";
}

export interface SettingsTab extends TabBase {
    type: "Settings";
}

export interface ConflictsTab extends TabBase {
    type: "Conflicts";
}

export type Tab = HomeTab | ALbumTab | SongTab | QueueTab | LyricsTab | SettingsTab | ConflictsTab;

export interface TabContextProps {
    currentTab: Tab;
    previousTab: Tab | null;
    tabHistory: Tab[];
    changeTo: (targetTab: Tab) => void;
    changeToPrevious: () => void;
}