// Keep in Sync with Rust PipelineParams struct.
// Must use snake case to match with Rust.
export default interface PipelineParams {
  out_file_type: "JPEG" | "WEBP";
  target_gamma: number;
  target_height: number;
  target_width: number;
  crop: PipelineCrop;
}

export interface PipelineCrop {
  x: number;
  y: number;
  width: number;
  height: number;
}
