import { getPromise } from "#lib/cachedPromise";
import { areDatesEqual, date_to_iso, iso_to_date } from "#lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { use, useEffect, useMemo, useState } from "react";
import { Button } from "./ui/button";
import { Calendar } from "./ui/calendar";
import { Spinner } from "./ui/spinner";

export function PhotoDateSelector({ onDateChange }: { onDateChange: (date: Date) => void }) {
  const [validDates, setValidDates] = useState<Date[]>();
  //   const validDates = useMemo(async () => {
  //     const validDates: string[] = await invoke("get_dates_with_photos");
  //     return validDates.map((str) => new Date(str));
  //   }, []);

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    const dates: string[] = await invoke("get_dates_with_photos");
    setValidDates(dates.map((str) => iso_to_date(str)));
  };

  if (!validDates) return <Spinner />;

  if (validDates.length === 0)
    return (
      <div>
        <p className="p-3 ">No database data found. Raws = 0</p>
        <Button onClick={fetchData}>Retry fetching data</Button>
      </div>
    );

  return <PhotoDateSelectorGated onDateChange={onDateChange} validDates={validDates!} />;
}

function PhotoDateSelectorGated({ onDateChange, validDates }: { onDateChange: (date: Date) => void; validDates: Date[] }) {
  const [selectedDate, setSelectedDate] = useState(validDates[validDates.length - 1]);

  const handleDateChange = (date: Date) => {
    setSelectedDate(date);
  };

  useEffect(() => {
    onDateChange(selectedDate);
  }, [selectedDate]);

  const disabledDate = (date: Date): boolean => {
    return typeof validDates.find((d) => areDatesEqual(date, d, "date_only")) === "undefined";
  };

  return (
    <>
      <h4>{selectedDate.toDateString()}</h4>
      <Calendar
        onSelect={handleDateChange}
        mode="single"
        fixedWeeks
        required
        selected={selectedDate}
        weekStartsOn={1}
        defaultMonth={selectedDate}
        startMonth={new Date(2024, 5)}
        endMonth={new Date()}
        disabled={disabledDate}
      />
    </>
  );
}
