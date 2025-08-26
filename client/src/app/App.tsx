import { Routes, Route, BrowserRouter } from 'react-router-dom';
import { HomePage } from '@/pages/HomePage';
import { Header } from '@/widgets/header';
import { AboutPage } from '@/pages/AboutPage';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <div>
        <Header />

        <main>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/about" element={<AboutPage />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  </StrictMode>
);
