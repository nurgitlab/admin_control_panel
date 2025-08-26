import React, { FC } from 'react';

import { useCounterStore } from '@/entities/CounterEntity/model/store';

export const Counter: FC = () => {
  const { count, increment, decrement, reset, setCount } = useCounterStore();

  return (
    <div
      style={{ padding: '20px', border: '1px solid #ccc', borderRadius: '8px' }}
    >
      <h2>Counter: {count}</h2>
      <div style={{ display: 'flex', gap: '10px', marginBottom: '10px' }}>
        <button onClick={decrement}>-</button>
        <button onClick={reset}>Reset</button>
        <button onClick={increment}>+</button>
      </div>
      <div>
        <input
          type="number"
          value={count}
          onChange={(e) => setCount(Number(e.target.value))}
          style={{ padding: '5px', width: '100px' }}
        />
      </div>
    </div>
  );
};
