import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Dispatch, SetStateAction, useEffect } from "react";
import { AudioRealtime } from "../lib/backendEvents";
import { Track } from "../lib/audioTypes";
import { PlayerState } from "../lib/playerActions";

/**
 * Well it's obvious right? Basically listens to events from backend.
 * Check out the AudioHandler in backend for better understanding.
* */
export const useBackendListener = (
    currentTrack: Track,
    setCurrentTrack: Dispatch<SetStateAction<Track>>,
    setPlayerState: Dispatch<SetStateAction<PlayerState>>
) => {
    // This will be most likely removed,
    // but it really depends how we handle the queue
    useEffect(() => {
        invoke("load_track", { path: currentTrack.path });
    }, []);
    useEffect(() => {
        let didSynced = false;
        let previousPos = 0;
        const progress = listen<AudioRealtime>("audio-realtime", (e) => {
            const { progress, info } = e.payload;
            // Most likely backend will read some settings from
            // last session, where the App will store things like
            // is muted, volume and so on
            if (!didSynced) {
                setPlayerState(s =>
                    s.setProgress(progress.position_ms / 1000)
                        .syncAudio(info.volume, info.muted)
                );
                didSynced = true;
            }
            else if (progress.finished)
                setPlayerState(s => s.setProgress(0).pause());
            else if (progress.position_ms !== previousPos) {
                previousPos = progress.position_ms;
                setPlayerState(s => s.setProgress(progress.position_ms / 1000));
            }
        });
        const loaded = listen<{ duration_ms: number | null }>("track-loaded", (e) => {
            const ms = e.payload.duration_ms;
            if (ms == null) return;
            setCurrentTrack(t => ({ ...t, duration: ms / 1000 }));
        });

        return () => {
            progress.then(f => f());
            loaded.then(f => f());
        };
    }, []);
};
