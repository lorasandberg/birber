import { date_to_iso } from "#lib/utils";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface WorkspaceState {
  selectedDate: string;
  openItem: { type: "photo" | "raw" | "species"; id: string } | null;
  loading: number;
}

const initialState: WorkspaceState = {
  selectedDate: date_to_iso(new Date()),
  openItem: null,
  loading: 0,
};

export const workspaceSlice = createSlice({
  name: "workspace",
  initialState,
  reducers: {
    setSelectedDate: (state, action: PayloadAction<string>) => {
      state.selectedDate = action.payload;
    },
    openRaw: (state, action: PayloadAction<string>) => {
      state.openItem = { type: "raw", id: action.payload };
    },
    closeItem: (state) => {
      state.openItem = null;
    },
    startLoading: (state) => {
      state.loading++;
    },
    finishLoading: (state) => {
      state.loading--;
    },
  },
});

export const { setSelectedDate, openRaw, closeItem, startLoading, finishLoading } = workspaceSlice.actions;

export default workspaceSlice.reducer;
