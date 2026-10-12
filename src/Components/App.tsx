import { useState } from "react";
import "../css/style.css";
import { Navbar } from "./General/Navbar";
import { Album, toTrack, Track } from "../lib/audioTypes";
import { MiniPlayer } from "./AudioPlayer/MiniPlayer";
import { PlayerState } from "../lib/playerActions";
import { miniPlayerBlacklist } from "../lib/miniPlayerBlacklist";
import { RenderTabs } from "./RenderTabs";
import { CurrPlayingContext } from "../lib/globalContext";
import { useActions } from "../hooks/useActions";
import { useBackendListener } from "../hooks/useBackendListener";
import { HomeTab, Tab, TABS } from '../lib/tabs';

const ALBUMS: Album[] = Array.from({ length: 9 }, (_, i) => ({
    id: String(i + 1),
    title: `Album ${i + 1}`,
    artist: [`Artist ${i + 1}`],
    cover: import.meta.env.VITE_TESTING_IMG
}));

function App() {
    const homeTab: HomeTab = { type: TABS.HOME };

    const [lastTab, setLastTab] = useState<Tab>(homeTab);
    const [currentTab, setCurrentTab] = useState<Tab>(homeTab);

    /// I know it's wrong but this is just something for a demo
    //  Also I have no fucking idea, how the DB will look
    const [currentTrack, setCurrentTrack] = useState<Track>(toTrack(ALBUMS[0], import.meta.env.VITE_TESTING_SOUND_LINK, 171));
    const [playerState, setPlayerState] = useState(new PlayerState);
    // const [hasConflicts] = useState(false);

    useBackendListener(
        currentTrack,
        setCurrentTrack,
        setPlayerState
    );

    const actions = useActions(
        setPlayerState,
        setLastTab,
        setCurrentTab,
        currentTab,
        lastTab,
        playerState
    );

    return (
        <CurrPlayingContext.Provider value={{
            // Behold, the context now has some actual context!
            // Other components can access this data.
            playerState: playerState,
            track: currentTrack,
            currentTab: currentTab,
            actions: actions,
        }}>

            <main className="min-h-screen text-foreground bg-background">
                <Navbar />
                <RenderTabs
                    albums={ALBUMS}
                />
                {miniPlayerBlacklist.includes(currentTab.type) ? <></> : <MiniPlayer />}
            </main>
        </CurrPlayingContext.Provider>
    );
}

export default App;
