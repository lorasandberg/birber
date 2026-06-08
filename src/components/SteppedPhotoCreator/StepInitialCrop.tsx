import { RawRecord } from "#components/RawItem";
import { Button } from "#components/ui/button";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useRef, useState } from "react";
import ReactCrop, { type Crop, type PixelCrop } from "react-image-crop";
import { PhotoEditParams } from "./SteppedPhotoCreator";

export default function StepInitialCrop({
  raw,
  params,
  setParams,
}: {
  raw: RawRecord;
  params: PhotoEditParams;
  setParams: (p: PhotoEditParams) => void;
}) {
  const [crop, setCrop] = useState<Crop>({
    unit: "%",
    x: 0,
    y: 0,
    width: 50,
    height: 50,
  });

  const [completedCrop, setCompletedCrop] = useState<PixelCrop | null>(null);
  const handleSaveCrop = () => {
    console.log(completedCrop);
  };
  const imgRef = useRef<HTMLImageElement>(null);
  const PREViEW_SIZE = { x: 600, y: 400 };

  function getTransformStyles(completedCrop: PixelCrop): string {
    // 1. Find out how much we need to scale up the image to make the cropped area fill the preview box
    const scaleX = PREViEW_SIZE.x / completedCrop.width;
    const scaleY = PREViEW_SIZE.y / completedCrop.height;

    // 2. Determine how many pixels we need to shift left/up to hide the unselected regions
    const translateX = -completedCrop.x * scaleX;
    const translateY = -completedCrop.y * scaleY;

    // 3. Return the composite GPU-optimized CSS matrix instruction
    return `translate3d(${translateX}px, ${translateY}px, 0) scale3d(${scaleX}, ${scaleY}, 1)`;
  }

  return (
    <div className="p-10 box-border h-full">
      <div className="flex flex-row">
        <div style={{ flex: "1 1 50%" }} className="p-10 flex justify-center items-center">
          <ReactCrop crop={crop} onComplete={setCompletedCrop} onChange={setCrop} aspect={3 / 2}>
            <img ref={imgRef} src={convertFileSrc(raw.jpg_path)} className="pointer-events-none select-none" style={{ aspectRatio: 3 / 2 }} />
          </ReactCrop>
        </div>
        <div style={{ flex: "1 1 50%" }} className="p-10 flex justify-center items-center">
          {completedCrop !== null && imgRef.current !== null && (
            <div className="overflow-hidden" style={{ width: `${PREViEW_SIZE.x}px`, height: `${PREViEW_SIZE.y}px` }}>
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

// {completedCrop && imgRef.current ? (
//           <div
//             style={{ width: PREVIEW_SIZE, height: PREVIEW_SIZE }}
//             className="overflow-hidden bg-neutral-950 rounded shadow-md border border-neutral-700 relative"
//           >
//             {/* The Invisible Magic Canvas Mirror */}
//             <img
//               src={imageSrc}
//               alt="Live Crop Preview"
//               style={{
//                 // Calculate scale multiplier based on how small the crop box is compared to the rendered image
//                 transform: getTransformStyles(completedCrop, imgRef.current, PREVIEW_SIZE),
//                 transformOrigin: 'top left',
//                 maxWidth: 'none', // Critical: prevents layout engines from crushing your dimensions
//               }}
//               // Match the exact base display width/height of your workspace target
//               width={imgRef.current.width}
//               height={imgRef.current.height}
//             />
//           </div>
//         ) : (
//           <div
//             style={{ width: PREVIEW_SIZE, height: PREVIEW_SIZE }}
//             className="bg-neutral-950 rounded border-2 border-dashed border-neutral-700 flex items-center justify-center text-xs text-neutral-500 text-center p-4"
//           >
//             Select an area to view preview
//           </div>
//         )}
