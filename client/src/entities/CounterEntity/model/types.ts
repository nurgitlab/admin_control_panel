export interface CounterState {
  count: number;
  increment: () => void;
  decrement: () => void;
  reset: () => void;
  setCount: (value: number) => void;
  message: string;
  setMessage: (value: string) => void;
}

export type CounterResponse = string;

export interface CounterParams {
  userId: number;
  userName: string;
}
