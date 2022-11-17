import { createContext, ReactNode, useContext, useState } from "react"

const GlobalContext = createContext({})

export const navLinks = [
  {
    name: "Home",
    path: "/",
  },
  {
    name: "Apps",
    path: "/apps",
  },
  {
    name: "Saves",
    path: "/saves",
  },
  {
    name: "Settings",
    path: "/settings",
  },
]

export const linkIndexMap = {
  Home: 0,
  Apps: 1,
  Saves: 2,
  Settings: 3,
}

export enum ServerState {
  init,
  ready,
  scraping,
  loading,
  syncing,
}

interface GlobalContextProps {
  children?: ReactNode
}

export const GlobalContextWrapper = ({ children }: GlobalContextProps) => {
  const [currentPage, setCurrentPage] = useState<PageName>()
  const [serverState] = useState<ServerState>()

  const sharedState = {
    currentPage,
    setCurrentPage,
    serverState,
  }
  return <GlobalContext.Provider value={sharedState}>{children}</GlobalContext.Provider>
}

export type PageName = "Home" | "Apps" | "Saves" | "Settings"
export type GlobalContextType = {
  currentPage: PageName
  setCurrentPage: (currentPage: PageName) => void
  serverState: ServerState
}

export const useGlobalContext = () => {
  return useContext(GlobalContext) as GlobalContextType
}
