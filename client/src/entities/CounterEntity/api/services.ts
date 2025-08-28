import { CounterResponse } from '../model';

export const counterService = {
  async getCounter(userId: number, userName: string): Promise<CounterResponse> {
    const response = await fetch(`/api/pingpong/${userId}/${userName}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Took took error');
    }

    const text = await response.text();

    return text;
  },
};
