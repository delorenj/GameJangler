import { listen } from "@tauri-apps/api/event"
import { createContext, ReactNode, useContext, useEffect, useState } from "react";

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
  const [serverMessages, setServerMessages] = useState<string[]>([])
  const [serverState] = useState<ServerState>()

  const sharedState = {
    currentPage,
    setCurrentPage,
    serverState,
    serverMessages,
  }

  const addMessage = (msg: string) => {
    const new_msg: string[] = serverMessages
    new_msg.push(msg)
    setServerMessages(new_msg)
  }

  const listener = async () => {
    console.log("about to await listen...")
    await listen("rs2js", (event: unknown) => {
      console.log(`Got message from server!: ${JSON.stringify(event)}`)
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      const input: string = event.payload
      console.log(`About to add message payload to serverMessages: ${input}`)
      addMessage(input)
    })
    console.log("After the await listen")
  }

  useEffect(() => {
    console.log("About to invoke listener()")
    listener().catch(e => `Listener promise exception: ${JSON.stringify(e)}`)
  }, [])

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
