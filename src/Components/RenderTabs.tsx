import { AlbumSection } from "./Sections/AlbumSection";
import { Conflicts } from "./Sections/Conflicts";
import { Home } from "./Sections/Home";
import { Lyrics } from "./Sections/Lyrics";
import { Settings } from "./Sections/Settings";
import { Song } from "./Sections/Song";
import { SongQueue } from "./Sections/SongQueue";
import { Album } from "../lib/audioTypes";
import { useContext } from "react";
import { ContextPlayerProps } from "../lib/contextType";
import { CurrPlayingContext } from "../lib/globalContext";
import { TABS } from '../lib/tabs';

export const RenderTabs = ({ albums }: { albums: Album[]; }) => {
    const playerInfo = useContext<ContextPlayerProps>(CurrPlayingContext);

    const currentTab = playerInfo.currentTab;

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
