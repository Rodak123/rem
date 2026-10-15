import { Album } from '../../lib/types/audioTypes';
import { cn } from '../../lib/utils';
import { vt } from '../../lib/viewTransition';

interface AlbumPreviewProps extends React.HTMLAttributes<HTMLDivElement> {
    album: Album;
    showTitle?: boolean;
    onClick?: () => void;
}

export const AlbumPreview: React.FC<AlbumPreviewProps> = ({ album, onClick, showTitle = true, ...props }) => {
    const { className } = props;

    return (
        <div {...props} className={cn(className, "album-item min-w-0 relative z-1")}>
            <div>
                <button
                    className={cn(
                        "aspect-square w-full rounded-lg object-cover my-1.75 border border-foreground/30",
                        onClick && "cursor-pointer hover:border-main"
                    )}
                    {...vt(`album-cover-${album.id}`, "Album")}
                    onClick={onClick}
                >
                    <img
                        src={album.cover}
                        alt={`Album cover for ${album.title}`}
                        className="aspect-square rounded-lg w-full album-cover"
                        loading="lazy"
                        decoding="async"
                    />
                </button>
                {showTitle &&
                    <h3
                        {...vt(`album-title-${album.id}`, "Album")}
                        className="mt-2 line-clamp-2 text-center text-lg sm:text-xl">
                        {album.title}
                    </h3>
                }
            </div>
            {onClick && <span className="hover-background absolute rounded-lg" />}
        </div>
    );
};
