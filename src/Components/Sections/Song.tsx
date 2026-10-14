import { Vinyl } from "./../AudioPlayer/Vinyl";
import { MdOutlineSkipNext, MdOutlineSkipPrevious, } from "react-icons/md";
import { IoPause, IoPlay } from "react-icons/io5";
import { ProgressBar } from "./../AudioPlayer/Progressbar";
import { MuteVolumeBtn } from "../AudioPlayer/MuteVolumeBtn";
import { usePlayerInfo } from '../../hooks/usePlayerInfo';

/**
    This component is for showing the currently played song in fullscreen.
    Miniplayer is for p*ssies, real men go here! Can be accessed using
    the render function from ./rem/src/lib/tabs.tsx
*/
export const Song = () => {
    //Loads all the data from App.tsx. When something changes there, this little guy will know.
    const playerInfo = usePlayerInfo();

    return (
        <div className="flex items-center justify-center flex-col w-full">
            <div className="flex items-center justify-center flex-col
            gap-6">
                <div style={{ viewTransitionName: "player-vinyl" } as React.CSSProperties}>
                    <Vinyl progress={playerInfo.playerState.progress}
                        track={playerInfo.track}
                        spinning={playerInfo.playerState.playing}
                        className="max-[350px]:w-50 max-[450px]:w-60 max-[600px]:w-80 w-110" />
                </div>
                <span className="min-w-0">
                    <span
                        style={{ viewTransitionName: "player-title" } as React.CSSProperties}
                        className="block truncate text-7xl
                    font-medium leading-tight">{playerInfo.track.title}</span>
                    <span
                        style={{ viewTransitionName: "player-author" } as React.CSSProperties}
                        className="block truncate text-xl text-muted">{playerInfo.track.author}</span>
                </span>
                <MuteVolumeBtn />
                <div className="w-full"
                    style={{ viewTransitionName: "progress-bar" } as React.CSSProperties}>
                    <ProgressBar current={playerInfo.playerState.progress}
                        duration={playerInfo.track.duration ?? 0}
                        onSeek={playerInfo.actions.setSeek}
                        className="stretch" />
                </div>
                <div className="flex items-center gap-4">
                    <button>
                        <MdOutlineSkipPrevious size={40} />
                    </button>
                    <button
                        onClick={playerInfo.actions.togglePlay}
                        aria-label={playerInfo.playerState.playing ? "Pause (P)" : "Play (P)"}
                        title={playerInfo.playerState.playing ? "Pause (P)" : "Play (P)"}
                        className="grid size-15 shrink-0 cursor-pointer place-items-center rounded-full
                        border border-foreground/40 transition-colors hover:bg-hover-bg"
                        style={{ viewTransitionName: "play-pause-btn" } as React.CSSProperties}
                    >
                        {playerInfo.playerState.playing ? <IoPause size={40} /> : <IoPlay size={40} />}
                    </button>
                    <button>
                        <MdOutlineSkipNext size={40} />
                    </button>
                </div>
            </div>

        </div>
    );
};
