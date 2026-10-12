import { Album } from '../../lib/audioTypes';
import { AlbumPreview } from '../General/AlbumPreview';

interface AlbumSectionProps {
    album: Album;
}

export const AlbumSection: React.FC<AlbumSectionProps> = ({ album }) => {
    return (
        <div style={{ width: '128px' }}>
            <AlbumPreview album={album} />
        </div>
    );
};
