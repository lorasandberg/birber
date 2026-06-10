import { PhotoDateSelector } from "#components/PhotoDateSelector";
import PhotoItem from "#components/PhotoItem";
import RawItem from "#components/RawItem";
import { Button } from "#components/ui/button";
import { ButtonGroup } from "#components/ui/button-group";
import { Spinner } from "#components/ui/spinner";
import { Switch } from "#components/ui/switch";
import { date_to_iso } from "#lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { Suspense, useEffect, useState } from "react";
import { PhotoRecord } from "./entities/PhotoRecord";
import { RawRecord } from "./entities/RawRecord";
import PhotoView from "./itemViews/PhotoView";
import RawView from "./itemViews/RawView";
import { useStacks } from "./StackManager";

export default function PhotoBrowser() {
  const { addStack, closeStack } = useStacks();
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);
  const [raws, setRaws] = useState<RawRecord[]>([]);
  const [photos, setPhotos] = useState<PhotoRecord[]>([]);
  const [showType, setShowType] = useState<"photo" | "raw">("photo");

  const [showBin, setShowBin] = useState<boolean>(false);

  useEffect(() => {
    if (selectedDate === null) return;
    (async () => {
      const rawRecords: RawRecord[] = await invoke("get_raws_by_date", { date: date_to_iso(selectedDate) });
      setRaws(rawRecords);

      const photoRecords: PhotoRecord[] = await invoke("get_photos_by_date", { date: date_to_iso(selectedDate) });
      setPhotos(photoRecords);
      //setOpenIndex(156);
    })();
  }, [selectedDate]);

  const openRaw = (index: number) => addStack(getRaw(index));
  const getRaw = (index: number) => (
    <RawView key={raws[index].cam_id} id={raws[index].cam_id} getNext={() => getRaw(index + 1)} getPrevious={() => getRaw(index - 1)} />
  );

  const openPhoto = (index: number) => addStack(getPhoto(index));
  const getPhoto = (index: number) => (
    <PhotoView key={photos[index].id} id={photos[index].id} getNext={() => getPhoto(index + 1)} getPrevious={() => getPhoto(index - 1)} />
  );

  const listPhotos = () => {
    return photos.map((photo, i) => {
      if (!showBin && photo.in_trash) return <></>;
      return <PhotoItem key={photo.id} photo={photo} onClick={() => openPhoto(i)} />;
    });
  };

  const listRaws = () => {
    return raws.map((raw, i) => {
      if (!showBin && raw.in_trash) return <></>;
      return <RawItem key={raw.id} raw={raw} onClick={() => openRaw(i)} />;
    });
  };

  return (
    <Suspense fallback={<Spinner />}>
      <div className={"flex flex-row max-h-full"}>
        <div className="flex-grow-1 p-3 overflow-auto">
          {selectedDate !== null && (
            <>
              <h1>{selectedDate.toLocaleDateString("en-CA", { weekday: "long", year: "numeric", month: "short", day: "numeric" })}</h1>

              <div className="flex flex-row flex-wrap gap-5 ">{showType === "photo" ? listPhotos() : listRaws()}</div>
            </>
          )}
        </div>

        <div className="flex flex-col p-3 gap-7">
          <div className="flex justify-between items-center">
            <ButtonGroup className="flex flex-between w-full">
              <Button variant={showType == "photo" ? "default" : "outline"} className="flex-grow-1" onClick={() => setShowType("photo")}>
                Photos
              </Button>
              <Button variant={showType == "raw" ? "default" : "outline"} className="flex-grow-1" onClick={() => setShowType("raw")}>
                Raws
              </Button>
            </ButtonGroup>
          </div>
          <PhotoDateSelector onDateChange={setSelectedDate} />
          <div className="flex justify-between items-center">
            <span>Show items in Bin</span>
            <Switch onCheckedChange={setShowBin} />
          </div>
        </div>
      </div>
    </Suspense>
  );
}
