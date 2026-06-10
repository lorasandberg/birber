import { convertFileSrc } from "@tauri-apps/api/core";
import { useRef, useState } from "react";
import ReactCrop, { Crop, PixelCrop } from "react-image-crop";
import { RawRecord } from "../entities/RawRecord";
import { Button } from "./ui/button";

export default function RawCrop({ raw, done }: { raw: RawRecord; done: (values: { x: number; y: number; width: number; height: number }) => void }) {
  const [crop, setCrop] = useState<Crop>({
    unit: "%",
    x: 0,
    y: 0,
    width: 50,
    height: 50,
  });

  const [completedCrop, setCompletedCrop] = useState<PixelCrop | null>(null);
  const handleSaveCrop = () => {
    if (imgRef.current === null) return;

    const width = imgRef.current.width;
    const height = imgRef.current.height;
    const round = (n: number) => Math.round(n * 100) / 100;
    done({ x: round(crop.x / width), y: round(crop.y / height), width: round(crop.width / width), height: round(crop.height / height) });
  };
  const imgRef = useRef<HTMLImageElement>(null);
  const PREVIEW_SIZE = { x: 600, y: 400 };

  function getTransformStyles(completedCrop: PixelCrop): string {
    // 1. Find out how much we need to scale up the image to make the cropped area fill the preview box
    const scaleX = PREVIEW_SIZE.x / completedCrop.width;
    const scaleY = PREVIEW_SIZE.y / completedCrop.height;

    // 2. Determine how many pixels we need to shift left/up to hide the unselected regions
    const translateX = -completedCrop.x * scaleX;
    const translateY = -completedCrop.y * scaleY;

    // 3. Return the composite GPU-optimized CSS matrix instruction
    return `translate3d(${translateX}px, ${translateY}px, 0) scale3d(${scaleX}, ${scaleY}, 1)`;
  }

  return (
    <div className="p-10 box-border h-full bg-black">
      <div className="flex flex-row">
        <div style={{ flex: "1 1 50%" }} className="p-10 flex justify-center items-center">
          <ReactCrop crop={crop} onComplete={setCompletedCrop} onChange={setCrop} aspect={3 / 2}>
            <img ref={imgRef} src={convertFileSrc(raw.jpg_path)} className="pointer-events-none select-none" style={{ aspectRatio: 3 / 2 }} />
          </ReactCrop>
        </div>
        <div style={{ flex: "1 1 50%" }} className="p-10 flex justify-center items-center">
          {completedCrop !== null && imgRef.current !== null && (
            <div className="overflow-hidden" style={{ width: `${PREVIEW_SIZE.x}px`, height: `${PREVIEW_SIZE.y}px` }}>
              <img
                src={convertFileSrc(raw.jpg_path)}
                style={{
                  transform: getTransformStyles(completedCrop),
                  transformOrigin: "top left",
                  maxWidth: "none",
                }}
                width={imgRef.current.width}
                height={imgRef.current.height}
              />
            </div>
          )}
        </div>
      </div>
      <Button onClick={handleSaveCrop}>Finish</Button>
    </div>
  );
}
