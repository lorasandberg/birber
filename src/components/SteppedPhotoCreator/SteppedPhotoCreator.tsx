/**
 * Step by step process for creating a photo from a RAW file.
 * Idea is that UI is stepped: user is lead through the process in logical steps which also supports photo creation.
 *
 * Steps:
 * - Initial crop
 * - Adjust crop
 * - Now we can get a RAW processed preview file
 *
 * - At this point, allow user to save and quit with some sensical defaults. (For fast "crop & tag & next" photo browsing)
 *
 * - Sliders to adjust gamma, contrast, exposure, highlights, shadows, color balance
 * - Allow user to go back to adjust crop?
 * - Save: what is saved is a preview file (max full HD?) with all settings saved.
 *   The user can now open the photo in the app and choose "Save -> Save with resolution" to get a final big file. (If needed)
 */

import { RawRecord } from "#components/RawItem";
import { useState } from "react";
import StepInitialCrop from "./StepInitialCrop";

export interface PhotoEditParams {
  crop?: { top: { x: number; y: number }; bottom: { x: number; y: number } };
}

export default function SteppedPhotoCreator({ raw }: { raw: RawRecord }) {
  const [params, setParams] = useState<PhotoEditParams>({});

  return (
    <div className="absolute w-full h-full top-0 left-0" style={{ backgroundColor: "rgba(0,0,0,90%)" }}>
      <StepInitialCrop raw={raw} params={params} setParams={setParams} />
    </div>
  );
}
