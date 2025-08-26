export interface CounterState {
  count: number;
  increment: () => void;
  decrement: () => void;
  reset: () => void;
  setCount: (value: number) => void;
}

export interface CounterActions {
  increment: () => void;
  decrement: () => void;
  reset: () => void;
  setCount: (value: number) => void;
}
