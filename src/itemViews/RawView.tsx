import { RawRecord } from "#components/RawItem";
import { Button } from "#components/ui/button";
import { Item } from "#components/ui/item";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { ArrowLeft, ArrowRight, Trash } from "lucide-react";
import { ReactNode, useEffect, useState } from "react";
import { TransformComponent, TransformWrapper } from "react-zoom-pan-pinch";
import { useStacks } from "../StackManager";
import PhotoView from "./PhotoView";

export default function RawView({
  id,
  getNext,
  getPrevious,
}: {
  id: string;
  getNext: null | (() => ReactNode);
  getPrevious: null | (() => ReactNode);
}) {
  const { closeStack, replaceStack, addStack } = useStacks();
  const [raw, setRaw] = useState<RawRecord | null>(null);
  const [loadCounter, setLoadCounter] = useState<number>(0);

  const [inTrash, setInTrash] = useState<boolean>(false);
  const [openEditor, setOpenEditor] = useState<boolean>(false);

  const getBinStatus = async () => {
    const result = JSON.parse(await invoke("get_bin_status", { camId: id }));
    setInTrash(parseInt(Object.values(result)[0] as string) === 0 ? false : true);
  };

  useEffect(() => {
    (async () => {
      const result: RawRecord = await invoke("get_raw_by_cam_id", { camId: id });
      setRaw(result);

      getBinStatus();
    })();
  }, [id, loadCounter]);

  if (raw === null) {
    return <p>Activity indicator..?</p>;
  }

  const onNext = () => {
    if (getNext === null) return;
    replaceStack(getNext());
  };

  const onPrevious = () => {
    if (getPrevious === null) return;
    replaceStack(getPrevious());
  };

  const thumbnail_path = `B:/Photos/_birber/thumbnails/${id}_thumbnail.jpg`;

  const createThumbnail = async () => {
    console.log("Triggering thumbnail creation...");
    const result = await invoke("trigger_create_thumbnail", { camId: id });

    console.log(result);
    setLoadCounter(loadCounter + 1);
  };

  const createPreview = async () => {
    console.log("Creating preview... *gulp*");
    const result = await invoke("create_preview_by_cam_id", { camId: id });
    console.log(result);
  };

  const createNewPhoto = () => {
    addStack(<PhotoView raw={raw} id={null} />);
  };

  const setToBeDeleted = async () => {
    console.log(await invoke("throw_out_raw", { camId: id }));
    getBinStatus();
    onNext();
  };

  return (
    <>
      <div className="flex flex-row h-full">
        <div className="p-1 flex flex-grow-1 bg-black justify-center items-center h-full">
          <div style={{ aspectRatio: 3 / 2, maxHeight: "100%" }}>
            <TransformWrapper
              initialScale={1}
              minScale={1}
              maxScale={32}
              wheel={{ step: 0.01 }}
              limitToBounds={true}
              velocityAnimation={{ disabled: true }}
              zoomAnimation={{ disabled: true }}
              autoAlignment={{ animationTime: 0 }}
            >
              <TransformComponent
                wrapperClass="!w-full !h-full flex items-center justify-center"
                contentClass="!w-full !h-full flex items-center justify-center"
              >
                <img style={{ width: "100%", height: "100%" }} src={convertFileSrc(raw.jpg_path)} />
              </TransformComponent>
            </TransformWrapper>
          </div>
        </div>
        <div className="px-5 flex gap-5 flex-col flex-grow-0">
          <div className="flex flex-row justify-between">
            <Button onClick={onPrevious} disabled={onPrevious === null}>
              <ArrowLeft />
            </Button>
            <Button onClick={closeStack}>Photos</Button>
            <Button onClick={onNext} disabled={onNext === null}>
              <ArrowRight />
            </Button>
          </div>
          <Item variant={"outline"}>
            <img style={{ width: "300px", height: "200px" }} src={convertFileSrc(thumbnail_path)} />
          </Item>
          <Button variant={"outline"} onClick={createNewPhoto}>
            Create new Photo
          </Button>
          <div className={"flex flex-grow-1"} />
          <div className="flex justify-between py-3 items-center gap-5">
            <span className="bold">{raw.cam_id}</span>
            <span className="text-red-700/100 bold">{inTrash ? "To be deleted" : ""}</span>
            <Button variant={"outline"} onClick={setToBeDeleted} style={{ backgroundColor: "rgb(216, 19, 19)" }}>
              <Trash className="size-5" color="white" />
            </Button>
          </div>
        </div>
      </div>
    </>
  );
}
