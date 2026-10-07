import { Album } from "../../lib/audioTypes";

export const Home = ({ albums }: { albums: Album[] }) => {
    return (
        <div className="md:flex md:flex-wrap grid grid-cols-2 pb-50">
            {
                albums.map((album) =>
                    <div key={album.id} className="p-3 w-fit">
                        <img className="w-80 h-80" src={album.cover} alt={`Album cover for ${album.title}`} />
                        <h3 className="text-center text-xl">{album.title}</h3>
                    </div>
                )
            }
        </div>
    );
};
