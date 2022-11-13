import { createContext, useContext, useEffect, useState } from "react"

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

export function GlobalContextWrapper({ children }) {
  const [currentPage, setCurrentPage] = useState()
  const [appData, setAppData] = useState()
  const [platformData, setPlatformData] = useState()
  const [gameData, setGameData] = useState()
  const [serverState, setServerState] = useState()

  const sharedState = {
    currentPage,
    setCurrentPage,
    appData,
    setAppData,
    platformData,
    setPlatformData,
    gameData,
    setGameData,
    serverState,
  }

  useEffect(() => {
    console.log("Setting/Getting server state...")
  }, [serverState, setServerState])

  return <GlobalContext.Provider value={sharedState}>{children}</GlobalContext.Provider>
}

export function useGlobalContext() {
  return useContext(GlobalContext)
}
