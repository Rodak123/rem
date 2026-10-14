import { invoke } from "@tauri-apps/api/core";
import { PlayerActions, PlayerState } from "../lib/playerActions";
import { Dispatch, SetStateAction } from "react";
import { withTransition } from "../lib/viewTransition";
import { useShortcuts } from "./useShortcuts";
import { useTabs } from './useTabs';
import { Album } from '../lib/types/audioTypes';

/**
 * Handles all input that happens during the runtime
 * And shortcuts too
 * */
export const useActions = (
    setPlayerState: Dispatch<SetStateAction<PlayerState>>,
    playerState: PlayerState
) => {
    const { currentTab, changeTo, changeToPrevious } = useTabs();

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
            changeTo({ type: "Album", album });
        },
        setHome: () => {
            changeTo({ type: "Home" });
        },
        setLyrics: () => {
            changeTo({ type: "Lyrics" });
        },
        setQueue: () => {
            changeTo({ type: "Song" });
        },
        setSong: () => {
            changeTo({ type: "Song" });
        },
        setPrevious: () => {
            /// Basically only to trigger the transition nothing insane
            withTransition(() => changeToPrevious());
        },
        setConflicts: () => {
            changeTo({ type: "Conflicts" });
        },
        setSettings: () => {
            changeTo({ type: "Settings" });
        },
    };

    const volumeUp = () => {
        actions.setVolume(playerState.volume + 0.05);
    };

    const volumeDown = () => {
        actions.setVolume(playerState.volume - 0.05);
    };

    const changeTab = () => {
        if (currentTab.type === "Home") {
            withTransition(() => actions.setSong());
        }
    };

    useShortcuts({
        r: actions.toggleRepeat,
        s: actions.toggleShuffle,
        l: actions.setLyrics,
        m: actions.toggleMute,
        q: actions.setQueue,
        p: actions.togglePlay,
        tab: changeTab,
        arrowup: volumeUp,
        arrowdown: volumeDown,
        /* This won't work most likely, because OS takes the media
        *  play pause key and thinks it's for different app
        *  This will be handled probably in Rust backend
        * */
        mediaplaypause: actions.togglePlay,
        " ": actions.togglePlay,
        o: actions.setSettings,
        "!": actions.setConflicts,
        escape: actions.setPrevious,
    });

    return actions;
};
