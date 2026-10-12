import { createContext } from "react";
import { ContextPlayerProps } from "../types/contextType";
import { Track } from "../types/audioTypes";
import { PlayerActions, PlayerState } from "../playerActions";

/**
    This stores important data about currently playing song. Every component using this context can access these data.
    I bet that this will need a more proper implementation tho.
    On the plus side, both Miniplayer and MainPlayer share resources this way.
    Here is just declaration, the data will be updated bellow.
*/
export const CurrPlayingContext = createContext<ContextPlayerProps>({
    track: null as unknown as Track,
    playerState: null as unknown as PlayerState,
    actions: null as unknown as PlayerActions,
});
