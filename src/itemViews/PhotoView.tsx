import RawCrop from "#components/RawCrop";
import { RawRecord } from "#components/RawItem";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import PipelineParams, { PipelineCrop } from "../entities/PipelineParameters";

export interface PhotoRecord {
  id: number;
  raw: RawRecord;
}

export default function PhotoView({ id, raw }: { id: string | null; raw?: RawRecord }) {
  const [photo, setPhoto] = useState<PhotoRecord | null>(null);

  const render = async (crop: PipelineCrop) => {
    const pipelineParams: PipelineParams = {
      crop: crop,
      target_gamma: 2.2,
      out_file_type: "JPEG",
      target_width: 1920,
      target_height: 1920 * (2 / 3),
    };

    const result = await invoke("create_new_photo", { camId: raw?.cam_id, pipelineParams: pipelineParams });

    console.log("???");
    console.log(result);
    console.log("?????");
    //Invoke create_photo (raw, crop)
    // Creates a DB row, returns id and other info.
  };

  if (id === null && raw !== null) {
    return <RawCrop raw={raw!} done={(crop) => render(crop)} />;
  }

  // Get photo info

  return (
    <div>
      <p>Photo vieewwww</p>
    </div>
  );
}
