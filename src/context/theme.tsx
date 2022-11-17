import React, { createContext, useContext } from "react"

const ThemeContext = createContext({})

interface ThemeContextProps {
  children?: React.ReactNode
}

export const ThemeContextWrapper = ({ children }: ThemeContextProps) => {
  return (
    <ThemeContext.Provider value={{}}>
      <div className="bg-white text-black dark:bg-slate-800 dark:text-white">
        {children}
      </div>
    </ThemeContext.Provider>
  )
}

export const useThemeContext = () => {
  return useContext(ThemeContext)
}
