import { ReactNode, useEffect } from 'react';
import { useUserStore } from '@/entities/UserEntitiy';

export const withStores = (component: () => ReactNode) => () => {
  const { initialize } = useUserStore();

  useEffect(() => {
    initialize();
  }, [initialize]);

  return <>{component()}</>;
};
