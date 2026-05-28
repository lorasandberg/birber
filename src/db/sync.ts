import { invoke } from "@tauri-apps/api/core";

/**
 * Finds all photo files and syncs database rows to match the files. Adds missing ones, removes removed ones.
 */
export const syncAll = async () => {
  console.log("Syncing");

  const results = await invoke("sync_all");

  //   let result: any[] = await invoke("list_unique_files");
  //   result = result.slice(0, 10);
  //   result = result.map((path: string) => convertFileSrc(path));

  //   console.log(result);
  console.log(results);
  console.log("Done");
};
