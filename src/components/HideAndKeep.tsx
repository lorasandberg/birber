import { ReactNode } from "react";

export default function HideAndKeep({ children, hide }: { children: ReactNode; hide: boolean }) {
  return (
    <div>
      <div className={hide ? "opacity-0 pointer-events-none absolute inset-0 -z-50 max-h-0 overflow-hidden" : ""}>{children}</div>
    </div>
  );
}
