import { Button } from "#components/ui/button";
import { ButtonGroup } from "#components/ui/button-group";
import { invoke } from "@tauri-apps/api/core";
import { useDispatch } from "react-redux";
import logo from "./assets/swallow.svg";
import { closeItem, finishLoading, startLoading } from "./store/workspaceSlice";

export default function Header() {
  const dispatch = useDispatch();

  const run_invoke = async (func_name: string) => {
    dispatch(startLoading());
    console.log(`Running '${func_name}'.`);
    const result = await invoke(func_name);
    console.log(result);
    dispatch(finishLoading());
  };

  return (
    <div className="flex flex-row gap-6 items-center p-5">
      <img src={logo} width={36} style={{ margin: "-5px", transform: "scaleX(-1)" }} />
      <ButtonGroup>
        <Button variant={"outline"} onClick={() => dispatch(closeItem())}>
          Photos
        </Button>
        <Button variant={"outline"}>Raws</Button>
      </ButtonGroup>
      <div className="flex-grow-1" />
      <Button variant={"outline"} onClick={() => run_invoke("create_all_missing_thumbnails")}>
        Create missing thumbnails
      </Button>
      <Button variant={"outline"} onClick={() => run_invoke("sync_all")}>
        Sync
      </Button>
    </div>
  );
}
