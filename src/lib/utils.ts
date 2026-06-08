import { clsx, type ClassValue } from "clsx";
import { useSelector } from "react-redux";
import { twMerge } from "tailwind-merge";
import { RootState } from "../store/store";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function date_to_iso(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}:${month}:${day}`;
}

export function iso_to_date(str: string): Date {
  return new Date(str.replace(/:/g, "/"));
}

export function useLoading() {
  return useSelector((state: RootState) => state.workspace.loading > 0);
}

export function areDatesEqual(d1: Date, d2: Date, level: "full" | "time_only" | "date_only"): boolean {
  switch (level) {
    case "full":
      return d1.toISOString() == d2.toISOString();
    case "date_only":
      return d1.toDateString() == d2.toDateString();
    case "time_only":
      return d1.toTimeString() == d2.toTimeString();
  }
}
