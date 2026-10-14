import { Album } from '../../lib/types/audioTypes';
import { AlbumPreview } from '../General/AlbumPreview';

interface AlbumSectionProps {
    album: Album;
}

export const AlbumSection: React.FC<AlbumSectionProps> = ({ album }) => {
    return (
        <div className="p-3 flex flex-col">
            <div className='w-full flex gap-4'>
                <AlbumPreview album={album} showTitle={false} className='grow' />
                <div className='grow w-1/2 flex flex-col'>
                    <h1 className='text-2xl p-1'>{album.title}</h1>
                    <div className='grow'>
                        <p className="text-lg">Short description about the album :P</p>
                    </div>
                    <div className='flex justify-around'>
                        <p>24 songs</p>
                        <p>1:25:15</p>
                    </div>
                </div>
            </div>
        </div>
    );
};
