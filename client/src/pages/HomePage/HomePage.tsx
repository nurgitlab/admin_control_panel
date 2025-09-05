import { FC } from 'react';

import { Counter } from '@/features/Counter';

export const HomePage: FC = () => {
  return (
    <div>
      <h1>Домашняя страница</h1>
      <p>Добро пожаловать в приложение!</p>
      <Counter />
    </div>
  );
};
