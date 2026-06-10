import RawCrop from "#components/RawCrop";
import { Spinner } from "#components/ui/spinner";
import ZoomView from "#components/ZoomView";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { ReactNode, useEffect, useState } from "react";
import { PhotoRecord } from "../entities/PhotoRecord";
import PipelineParams, { PipelineCrop } from "../entities/PipelineParameters";
import { RawRecord } from "../entities/RawRecord";

export default function PhotoView({
  id,
  raw,
  getNext,
  getPrevious,
}: {
  id: number | null;
  raw?: RawRecord;
  getNext?: null | (() => ReactNode);
  getPrevious?: null | (() => ReactNode);
}) {
  const [photo, setPhoto] = useState<PhotoRecord | null>(null);

  const render = async (crop: PipelineCrop) => {
    const pipelineParams: PipelineParams = {
      crop: crop,
      target_gamma: 2.2,
      out_file_type: "JPEG",
      target_width: 1920,
      target_height: 1920 * (2 / 3),
    };

    const result: PhotoRecord = await invoke("create_new_photo", { camId: raw?.cam_id, pipelineParams: pipelineParams });

    setPhoto(result);
  };

  useEffect(() => {
    loadById();
  }, []);

  const loadById = async () => {
    if (id === null) return;
    setPhoto(await invoke("get_photo_by_id", { id: id }));
  };

  if (id === null && raw !== null && photo === null) {
    return <RawCrop raw={raw!} done={(crop) => render(crop)} />;
  }

  // Get photo info

  if (photo === null) return <Spinner />;

  return (
    <div className="flex flex-1 h-full">
      <ZoomView image_path={convertFileSrc(photo.file_path)} />
      <div className="w-100">hello sidebar</div>
    </div>
  );
}
