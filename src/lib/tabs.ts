import { Album } from './audioTypes';

export const TABS = {
    HOME: "Home",
    ALBUM: "Album",
    SONG: "Song",
    QUEUE: "Queue",
    LYRICS: "Lyrics",
    SETTINGS: "Settings",
    CONFLICTS: "Conflicts",
} as const;

export type TabType = typeof TABS[keyof typeof TABS];

interface TabBase {
    type: TabType;
}

export interface HomeTab extends TabBase {
    type: typeof TABS["HOME"];
}

export interface ALbumTab extends TabBase {
    type: typeof TABS["ALBUM"];
    album: Album,
}

export interface SongTab extends TabBase {
    type: typeof TABS["SONG"];
}

export interface QueueTab extends TabBase {
    type: typeof TABS["QUEUE"];
}

export interface LyricsTab extends TabBase {
    type: typeof TABS["LYRICS"];
}

export interface SettingsTab extends TabBase {
    type: typeof TABS["SETTINGS"];
}

export interface ConflictsTab extends TabBase {
    type: typeof TABS["CONFLICTS"];
}

export type Tab = HomeTab | ALbumTab | SongTab | QueueTab | LyricsTab | SettingsTab | ConflictsTab;