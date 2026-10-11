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

export type Tabs = "Home" | "Album" | "Song" | "Queue" | "Lyrics" | "Settings" | "Conflicts";
export const RenderTabs = ({ albums }: { albums: Album[] }) => {
    const playerInfo = useContext<ContextPlayerProps>(CurrPlayingContext);
    switch (playerInfo.currentTab) {
        case "Home":
            return <Home albums={albums} />;
        case "Song":
            return <Song />;
        case "Album":
            return <AlbumSection />;
        case "Queue":
            return <SongQueue />;
        case "Lyrics":
            return <Lyrics />;
        case "Conflicts":
            return <Conflicts />;
        case "Settings":
            return <Settings />;
    }
};
