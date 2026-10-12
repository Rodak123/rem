import { TABS, TabType } from './types/tabs';

/*
This is the app is on one of there, the mini player will NOT show! These components like to be alone.
*/
export const miniPlayerBlacklist: TabType[] = [
    TABS.SONG, TABS.SETTINGS, TABS.CONFLICTS, TABS.LYRICS
];
