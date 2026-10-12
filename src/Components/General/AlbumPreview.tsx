import { useContext } from 'react';
import { Album } from '../../lib/audioTypes';
import { ContextPlayerProps } from '../../lib/contextType';
import { CurrPlayingContext } from '../../lib/globalContext';

interface AlbumPreviewProps {
    album: Album;
}

export const AlbumPreview: React.FC<AlbumPreviewProps> = ({ album }) => {
    const playerInfo = useContext<ContextPlayerProps>(CurrPlayingContext);

    return (
        <div className="album-item min-w-0 relative z-1 m-0.75">
            <div>
                <button
                    className="cursor-pointer aspect-square w-full rounded-lg object-cover border border-foreground/30 hover:border-main"
                    onClick={() => playerInfo.actions.setAlbumSection(album)}
                >
                    <img
                        src={album.cover}
                        alt={`Album cover for ${album.title}`}
                        className="aspect-square rounded-lg w-full"
                        loading="lazy"
                        decoding="async"
                    />
                </button>
                <h3 className="mt-2 line-clamp-2 text-center text-lg sm:text-xl">
                    {album.title}
                </h3>
            </div>
            <span className="hover-background absolute rounded-lg" />
        </div>
    );
};