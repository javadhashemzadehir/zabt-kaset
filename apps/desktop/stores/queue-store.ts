import { create } from "zustand";
import type { SourceType } from "@/types/api";

interface QueueUiState {
  selectedIds: string[];
  draftUrls: string;
  presetId: string;
  sourceType: SourceType;
  setDraftUrls: (value: string) => void;
  setPresetId: (value: string) => void;
  setSourceType: (value: SourceType) => void;
  toggleSelected: (id: string) => void;
  clearSelected: () => void;
}

export const useQueueStore = create<QueueUiState>((set) => ({
  selectedIds: [],
  draftUrls: "",
  presetId: "preset-balanced",
  sourceType: "video",
  setDraftUrls: (draftUrls) => set({ draftUrls }),
  setPresetId: (presetId) => set({ presetId }),
  setSourceType: (sourceType) => set({ sourceType }),
  toggleSelected: (id) =>
    set((state) => ({
      selectedIds: state.selectedIds.includes(id)
        ? state.selectedIds.filter((x) => x !== id)
        : [...state.selectedIds, id],
    })),
  clearSelected: () => set({ selectedIds: [] }),
}));
