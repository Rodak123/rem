export type Album = {
    id: string;
    title?: string;
    artist: string[];
    cover?: string;
};

export type Track = {
    id: string;
    title: string;
    comment: string;
    path: string;
    duration: number;
    author: string[];
    cover?: number[];
    if_favorite: boolean;
};
