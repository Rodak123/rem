import { invoke } from "@tauri-apps/api/core";
import { PlayerActions, PlayerState } from "../lib/playerActions";
import { Dispatch, SetStateAction } from "react";
import { withTransition } from "../lib/viewTransition";
import { useShortcuts } from "./useShortcuts";
import { Tab, TABS } from '../lib/tabs';
import { Album } from '../lib/audioTypes';

/**
 * Handles all input that happens during the runtime
 * And shortcuts too
 * */
export const useActions = (
    setPlayerState: Dispatch<SetStateAction<PlayerState>>,
    setLastTab: Dispatch<SetStateAction<Tab>>,
    setCurrentTab: Dispatch<SetStateAction<Tab>>,
    currentTab: Tab,
    lastTab: Tab,
    playerState: PlayerState
) => {
    const actions: PlayerActions = {
        togglePlay: () => {
            invoke("toggle");
            setPlayerState(s => s.togglePlay());
        },
        toggleRepeat: () => setPlayerState(s => s.toggleRepeat()),
        toggleShuffle: () => setPlayerState(s => s.toggleShuffle()),
        toggleMute: () => {
            invoke("mute").then(() => {
                setPlayerState(s => s.toggleMute());
            });
        },
        setSeek: (num: number) => {
            invoke("set_seek", { duration: num });
            setPlayerState(s => s.setProgress(num));
        },
        setVolume: (num: number) => {
            const volume = Math.min(1, Math.max(0, num));
            invoke("set_volume", { volume })
                .then(() => invoke("set_mute", { muted: false }))
                .then(() => setPlayerState(s => s.setVolume(volume).unmute()));
        },
        pressShuffle: () => { /* This is going to be little longer
            cuz it requires also saving the state before shuffle*/ },
        setAlbumSection: (album: Album) => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.ALBUM, album });
        },
        setHome: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.HOME });
        },
        setLyrics: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.LYRICS });
        },
        setQueue: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.QUEUE });
        },
        setSong: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.SONG });
        },
        setPrevious: () => {
            /// Basically only to trigger the transition nothing insane
            withTransition(() => setCurrentTab(lastTab));
            setLastTab(currentTab);
        },
        setConflicts: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.CONFLICTS });
        },
        setSettings: () => {
            setLastTab(currentTab);
            setCurrentTab({ type: TABS.SETTINGS });
        },
    };

    useShortcuts({
        r: actions.toggleRepeat,
        s: actions.toggleShuffle,
        l: actions.setLyrics,
        m: actions.toggleMute,
        q: actions.setQueue,
        p: actions.togglePlay,
        arrowup: () => {
            console.log("up");
            actions.setVolume(playerState.volume + 0.05);
        },
        arrowdown: () => actions.setVolume(playerState.volume - 0.05),
        /* This won't work most likely, because OS takes the media
        *  play pause key and thinks it's for different app
        *  This will be handled probably in Rust backend
        * */
        mediaplaypause: actions.togglePlay,
        " ": actions.togglePlay,
        o: actions.setSettings,
        "!": actions.setConflicts,
        escape: () => currentTab.type !== TABS.HOME ? withTransition(() => actions.setHome()) : {},
    });

    return actions;
};
