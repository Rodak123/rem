export type Album = {
    id: string;
    title?: string;
    artist: string[];
    cover?: string;
    trackCount: number;
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
    diskNumber: number;
};

export type AlbumDisk = {
    number: number;
    trackCount: number;
};