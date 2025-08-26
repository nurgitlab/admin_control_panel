import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter, Route, Routes } from 'react-router-dom';

import { AboutPage } from '@/pages/AboutPage';
import { HomePage } from '@/pages/HomePage';
import { Header } from '@/widgets/header';

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
