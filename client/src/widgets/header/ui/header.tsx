import { useUserStore } from '@/entities/UserEntitiy'

export const Header = () => {
  const { user, logout } = useUserStore()
  
  return (
    <header>
      <nav>
        <div>Логотип</div>
        <div>
          {user ? (
            <button onClick={logout}>Выйти</button>
          ) : (
            <span>Гость</span>
          )}
        </div>
      </nav>
    </header>
  )
}