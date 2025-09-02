import { FC, useState } from 'react';

import { useCounterMutation } from '@/entities/CounterEntity/api/hooks';
import { useCounterStore } from '@/entities/CounterEntity/model/store';

export const Counter: FC = () => {
  const { count, increment, decrement, reset, message, setCount } =
    useCounterStore();

  const { mutate, isPending, isError, error } = useCounterMutation();

  const [userId, setUserId] = useState(0);
  const [userName, setUserName] = useState('');

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

      <div style={{ display: 'flex', gap: '10px', marginBottom: '10px' }}>
        <input
          type="number"
          value={userId}
          onChange={(e) => setUserId(Number(e.target.value))}
          style={{ padding: '5px', width: '100px' }}
        />

        <input
          type="string"
          value={userName}
          onChange={(e) => setUserName(e.target.value)}
          style={{ padding: '5px', width: '100px' }}
        />
        <button onClick={() => mutate({ userId, userName })}>
          {isPending ? 'Loading...' : 'SEND PING PONG'}
        </button>
      </div>

      {isError && (
        <div style={{ color: 'red', marginTop: '10px' }}>
          Ошибка: {error.message}
        </div>
      )}

      <div>{message}</div>
    </div>
  );
};
