import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import { CounterState } from './types';

export const useCounterStore = create(
  immer<CounterState>((set) => ({
    count: 0,
    message: '',
    increment: () =>
      set((state) => {
        state.count += 1;
      }),
    decrement: () =>
      set((state) => {
        state.count -= 1;
      }),
    reset: () =>
      set((state) => {
        state.count = 0;
      }),
    setCount: (value: number) =>
      set((state) => {
        state.count = value;
      }),
    setMessage: (message: string) =>
      set((state) => {
        state.message = message;
      }),
  }))
);
