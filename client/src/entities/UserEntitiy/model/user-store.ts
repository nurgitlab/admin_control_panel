import { create } from 'zustand'
import { devtools } from 'zustand/middleware'

interface User {
  id: string
  email: string
  name: string
}

interface UserStore {
  user: User | null
  isAuthenticated: boolean
  isLoading: boolean
  setUser: (user: User | null) => void


  initialize: () => Promise<void>
  logout: () => void
}

export const useUserStore = create<UserStore>()(
  devtools(
    (set) => ({
      user: null,
      isAuthenticated: false,
      isLoading: true,
      
      setUser: (user) => set({ 
        user, 
        isAuthenticated: !!user,

        
        isLoading: false 
      }),
      
      logout: () => {
        localStorage.removeItem('token')
        set({ 
          user: null, 
          isAuthenticated: false 
        })
      },
    }),
    { name: 'UserStore' }
  )
)