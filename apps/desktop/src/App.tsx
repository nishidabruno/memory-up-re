import './styles/global.css'

import { ThemeProvider } from '@components/theme-provider'

import { Home } from './pages/home'

function App() {
  return (
    <ThemeProvider defaultTheme="system" storageKey="memoryup-theme">
      <Home />
    </ThemeProvider>
  );
}

export default App;
