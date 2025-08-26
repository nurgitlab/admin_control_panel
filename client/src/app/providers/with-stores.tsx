import { ReactNode, useEffect } from 'react';

import { useUserStore } from '@/entities/UserEntitiy';

// eslint-disable-next-line react/display-name
export const withStores = (component: () => ReactNode) => () => {
  const { initialize } = useUserStore();

  useEffect(() => {
    initialize();
  }, [initialize]);

  return <>{component()}</>;
};
