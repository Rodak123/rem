import { invoke } from '@tauri-apps/api/core';
import { usePlayerInfo } from '../../hooks/usePlayerInfo';
import { Album, Track } from "../../lib/types/audioTypes";
import { withTransition } from '../../lib/viewTransition';
import { AlbumPreview } from '../General/AlbumPreview';
import { useEffect, useState } from 'react';

export const Home = ({ albums }: { albums: Album[]; }) => {
    const playerInfo = usePlayerInfo();
    const [data, setData] = useState<Track[]>([]);
    useEffect(() => {
        const process = async () => {
            const d: Track[] = await invoke("get_all_tracks");
            setData(d);
        };
        process();
    }, []);


    return (
        <div>
            <div className='flex flex-col'>
                {data.map(track =>
                    <button
                        className='p-3 bg-background-dark m-2
                        hover:bg-background-light'
                        key={track.id}
                        onClick={() => {
                            playerInfo.actions.loadSong(track.path, true);
                        }}
                    >{track.title}
                    </button>
                )}
            </div>
            <div className="grid grid-cols-[repeat(auto-fill,minmax(min(100%,10rem),1fr))] gap-4 p-3 pb-50 sm:gap-6">
                {albums.map((album) => (
                    <AlbumPreview
                        key={album.id}
                        album={album}
                        className="album-item"
                        onClick={() => withTransition(() => {
                            playerInfo.actions.setAlbumSection(album);
                        }, "Album")}
                    />
                ))}
            </div>
        </div>
    );
};
