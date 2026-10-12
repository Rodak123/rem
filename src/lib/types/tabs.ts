import { Album } from './audioTypes';

// Each tab holds its type and some of its own data,
// which specifies what the tab will show

export type TabType = "Home" | "Album" | "Song" | "Queue" | "Lyrics" | "Settings" | "Conflicts";

interface TabBase {
    type: TabType;
}

export interface HomeTab extends TabBase {
    type: "Home";
}

export interface AlbumTab extends TabBase {
    type: "Album";
    // This will change to albumId
    // and the tab will fetch it from the backend
    album: Album;
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

export type Tab = HomeTab | AlbumTab | SongTab | QueueTab | LyricsTab | SettingsTab | ConflictsTab;

export interface TabContextProps {
    currentTab: Tab;
    previousTab: Tab | null;
    tabHistory: Tab[];
    changeTo: (targetTab: Tab) => void;
    changeToPrevious: () => void;
}