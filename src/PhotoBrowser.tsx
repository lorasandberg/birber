import { PhotoDateSelector } from "#components/PhotoDateSelector";
import RawItem, { RawRecord } from "#components/RawItem";
import { Spinner } from "#components/ui/spinner";
import { date_to_iso } from "#lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { Suspense, useEffect, useState } from "react";
import RawView from "./itemViews/RawView";
import { useStacks } from "./StackManager";

export default function PhotoBrowser() {
  const { addStack, closeStack } = useStacks();
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);
  const [raws, setRaws] = useState<RawRecord[]>([]);

  useEffect(() => {
    if (selectedDate === null) return;
    (async () => {
      const result: RawRecord[] = await invoke("get_raws_by_date", { date: date_to_iso(selectedDate) });
      setRaws(result);
      //setOpenIndex(156);
    })();
  }, [selectedDate]);

  const getItem = (index: number) => {
    return <RawView key={raws[index].cam_id} id={raws[index].cam_id} getNext={() => getItem(index + 1)} getPrevious={() => getItem(index - 1)} />;
  };

  const openView = (index: number) => {
    addStack(getItem(index));
  };

  return (
    <Suspense fallback={<Spinner />}>
      <div className={"flex flex-row max-h-full"}>
        <div className="flex-grow-1 p-3 overflow-auto">
          {selectedDate !== null && (
            <>
              <h1>{selectedDate.toLocaleDateString("en-CA", { weekday: "long", year: "numeric", month: "short", day: "numeric" })}</h1>

              <div className="flex flex-row flex-wrap gap-5 ">
                {raws.map((raw, i) => (
                  <RawItem key={raw.id} raw={raw} onClick={() => openView(i)} />
                ))}
              </div>
            </>
          )}
        </div>

        <div className="flex flex-col p-3">
          <PhotoDateSelector onDateChange={setSelectedDate} />
        </div>
      </div>
    </Suspense>
  );
}
