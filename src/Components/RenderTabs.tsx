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
        case "Home":
            return <Home albums={albums} />;
        case "Song":
            return <Song />;
        case "Album":
            return <AlbumSection album={currentTab.album} />;
        case "Song":
            return <SongQueue />;
        case "Lyrics":
            return <Lyrics />;
        case "Conflicts":
            return <Conflicts />;
        case "Settings":
            return <Settings />;
    }
};
