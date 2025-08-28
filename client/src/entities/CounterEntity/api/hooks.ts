import { useMutation } from '@tanstack/react-query';

import { CounterParams, CounterResponse } from '../model';
import { useCounterStore } from '../model/store';
import { counterService } from './services';

export const useCounterMutation = () => {
  const { setMessage } = useCounterStore();

  const mutation = useMutation<CounterResponse, Error, CounterParams>({
    mutationFn: ({ userId, userName }: CounterParams) =>
      counterService.getCounter(userId, userName),
    onSuccess: (data: CounterResponse) => {
      //   setCount(count + 1);
      setMessage(data);
    },
  });

  return mutation;
};
