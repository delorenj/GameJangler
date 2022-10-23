import { createContext, useContext } from "react";

const ThemeContext = createContext({});

export function ThemeContextWrapper({ children }) {
  return (
    <ThemeContext.Provider value={{}}>
      <div className="dark:text-white dark:bg-slate-800 bg-white text-black">
        {children}
      </div>
    </ThemeContext.Provider>
  );
}

export function useThemeContext() {
  return useContext(ThemeContext);
}
