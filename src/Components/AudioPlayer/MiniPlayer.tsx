import { IoPause, IoPlay } from "react-icons/io5";
import { Vinyl } from "./Vinyl";
import { useSwipe } from "../../hooks/useSwipe";
import { ProgressBar } from "./Progressbar";
import { ContextPlayerProps } from "../../lib/contextType";
import { useContext } from "react";
import { CurrPlayingContext } from "../../lib/globalContext";
import { withTransition } from "../../lib/viewTransition";
import { MuteVolumeBtn } from "./MuteVolumeBtn";

export const MiniPlayer = () => {
    const miniPlayerInfo = useContext<ContextPlayerProps>(CurrPlayingContext);
    const expand = () => withTransition(miniPlayerInfo.actions.setSong);
    // Swipe up (or tap) on the vinyl / title area pulls the full player up
    const swipe = useSwipe({ onUp: miniPlayerInfo.actions.setSong });

    return (
        <div className="rounded-3xl border
            fixed bottom-4 w-21/22
            bg-background-dark
            left-1/2
            -translate-x-1/2
            border-foreground/30 bg-card-bg p-3 z-3">
            <div className="flex items-center gap-3">
                <button
                    {...swipe}
                    onClick={expand}
                    aria-label="Open player"
                    className="flex min-w-0 flex-1
                    cursor-pointer touch-none items-center gap-3 text-left"
                >
                    {/*
                        Hopefully this is your first encounter with `viewTransitionName`, else ... sorry
                        Anyways you can add this name to element for sick transition from MiniPlayer into
                        Song Tab. You just need put the same name of elememnts and webview will do the rest
                    */}
                    <div style={{ viewTransitionName: "player-vinyl" } as React.CSSProperties}>
                        <Vinyl progress={miniPlayerInfo.playerState.progress}
                            track={miniPlayerInfo.track}
                            spinning={miniPlayerInfo.playerState.playing}
                            className="w-14" />
                    </div>
                    <span className="min-w-0">
                        <span
                            style={{ viewTransitionName: "player-title" } as React.CSSProperties}
                            className="block truncate text-lg font-medium
                        leading-tight">{miniPlayerInfo.track.title}</span>
                        <span
                            style={{ viewTransitionName: "player-author" } as React.CSSProperties}
                            className="block truncate text-sm text-muted">
                            {miniPlayerInfo.track.author}</span>
                    </span>
                </button>

                <MuteVolumeBtn />

                <button
                    onClick={miniPlayerInfo.actions.togglePlay}
                    aria-label={miniPlayerInfo.playerState.playing
                        ? "Pause (P)" : "Play (P)"}
                    title={miniPlayerInfo.playerState.playing
                        ? "Pause (P)" : "Play (P)"}
                    className="grid size-11 shrink-0 cursor-pointer
                    place-items-center rounded-full border
                    border-foreground/40 transition-colors
                    hover:bg-hover-bg"
                    style={{ viewTransitionName: "play-pause-btn" } as React.CSSProperties}
                >
                    {miniPlayerInfo.playerState.playing ? <IoPause size={20} /> : <IoPlay size={20} />}
                </button>
            </div>

            <div style={{ viewTransitionName: "progress-bar" } as React.CSSProperties}>
                <ProgressBar current={miniPlayerInfo.playerState.progress}
                    duration={miniPlayerInfo.track.duration ?? 0}
                    onSeek={miniPlayerInfo.actions.setSeek} className="mt-3" />
            </div>
        </div>
    );
};
