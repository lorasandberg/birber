import { convertFileSrc } from "@tauri-apps/api/core";
import { Trash } from "lucide-react";
import { PhotoRecord } from "../entities/PhotoRecord";
import { Item } from "./ui/item";

export default function PhotoItem({ photo, onClick }: { photo: PhotoRecord; onClick: () => void }) {
  const handleClick = () => {
    onClick();
  };

  return (
    <Item
      style={{ maxWidth: "300px" }}
      variant={"outline"}
      asChild
      className={"cursor-pointer relative " + (photo.in_trash && "border-5 border-red-400")}
    >
      <a onClick={handleClick}>
        {photo.in_trash && (
          <div className="absolute bg-red-600/80 rounded-full p-[6px] bottom-[19px] right-[21px]">
            <Trash
              size={14}
              color="white
            "
            />
          </div>
        )}
        <img src={convertFileSrc(photo.thumbnail)} style={{ maxWidth: "100%" }} />
      </a>
    </Item>
  );
}
