export type AudioRealtime = {
    progress: { position_ms: number; finished: boolean };
    info: { volume: number; muted: boolean };
};
