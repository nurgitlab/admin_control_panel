import { CounterResponse } from '../model';

export const counterService = {
  async getCounter(userId: number, userName: string): Promise<CounterResponse> {
    // eslint-disable-next-line no-console
    console.log('here');
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
