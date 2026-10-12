import { AlbumSection } from "./Sections/AlbumSection";
import { Conflicts } from "./Sections/Conflicts";
import { Home } from "./Sections/Home";
import { Lyrics } from "./Sections/Lyrics";
import { Settings } from "./Sections/Settings";
import { Song } from "./Sections/Song";
import { SongQueue } from "./Sections/SongQueue";
import { Album } from "../lib/types/audioTypes";
import { TABS } from '../lib/types/tabs';
import { useTabs } from '../hooks/useTabs';

export const RenderTabs = ({ albums }: { albums: Album[]; }) => {
    const { currentTab } = useTabs();

    switch (currentTab.type) {
        case TABS.HOME:
            return <Home albums={albums} />;
        case TABS.SONG:
            return <Song />;
        case TABS.ALBUM:
            return <AlbumSection album={currentTab.album} />;
        case TABS.QUEUE:
            return <SongQueue />;
        case TABS.LYRICS:
            return <Lyrics />;
        case TABS.CONFLICTS:
            return <Conflicts />;
        case TABS.SETTINGS:
            return <Settings />;
    }
};
