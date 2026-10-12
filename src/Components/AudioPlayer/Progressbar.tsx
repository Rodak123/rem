import { CSSProperties } from "react";
import "../../css/progressBar.css";
import { TABS } from '../../lib/types/tabs';
import { useTabs } from '../../hooks/useTabs';

const format = (seconds: number) =>
    `${Math.floor(seconds / 60)}:${String(Math.floor(seconds % 60)).padStart(2, "0")}`;

type ProgressBarProps = {
    current: number;
    duration: number;
    onSeek: (seconds: number) => void;
    className?: string;
};

export const ProgressBar = ({ current, duration, onSeek, className = "" }: ProgressBarProps) => {
    const { currentTab } = useTabs();

    const isSongTab = currentTab.type === "Song";
    const percent = duration > 0 ? (current / duration) * 100 : 0;
    return (
        <div className={`flex gap-2 text-[10px]
            ${isSongTab ? "flex-col" : "items-center"}
            tabular-nums text-muted ${className}`}>
            <span className={isSongTab ? "hidden" : ""}>{format(current)}</span>
            <input
                type="range"
                min={0}
                max={duration}
                step={1}
                value={current}
                onChange={(e) => onSeek(Number(e.target.value))}
                aria-label="Seek"
                style={{ "--progress": `${percent}%` } as CSSProperties}
                className="progress-range flex-1 w-full min-h-2"
            />
            <div className="flex justify-between">
                <span className={isSongTab ? "" : "hidden"}>{format(current)}</span>
                <span>{format(duration)}</span>
            </div>
        </div>
    );
};
