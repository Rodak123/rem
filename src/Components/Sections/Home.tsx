import { Album } from "../../lib/audioTypes";
import { AlbumPreview } from '../General/AlbumPreview';

export const Home = ({ albums }: { albums: Album[]; }) => {
    return (
        <div className="grid grid-cols-[repeat(auto-fill,minmax(min(100%,10rem),1fr))] gap-4 p-3 pb-50 sm:gap-6">
            {albums.map((album) => (
                <AlbumPreview key={album.id} album={album} />
            ))}
        </div>
    );
};
