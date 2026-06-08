import { convertFileSrc } from "@tauri-apps/api/core";
import { Item } from "./ui/item";

export interface RawRecord {
  id: number;
  cam_id: string;
  raw_path: string;
  jpg_path: string;
  date_taken: string;
  thumbnail: string;
}

export default function RawItem({ raw, onClick }: { raw: RawRecord; onClick: () => void }) {
  const thumbnail_path = `B:/Photos/_birber/thumbnails/${raw.cam_id}_thumbnail.jpg`;

  const handleClick = () => {
    onClick();
  };

  return (
    <Item style={{ maxWidth: "300px" }} variant={"outline"} asChild className="cursor-pointer">
      <a onClick={handleClick}>
        <img src={convertFileSrc(thumbnail_path)} style={{ maxWidth: "100%" }} />
      </a>
    </Item>
  );
}
