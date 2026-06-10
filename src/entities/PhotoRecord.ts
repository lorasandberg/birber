import { RawRecord } from "../entities/RawRecord";
import PipelineParams from "./PipelineParameters";

export interface PhotoRecord {
  id: number;
  raw: RawRecord;
  file_path: string;
  pipeline_params: PipelineParams;
  rating: string;
  in_trash: boolean;
  thumbnail: string;
}
