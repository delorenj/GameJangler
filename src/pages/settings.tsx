import { invoke } from "@tauri-apps/api"
import type { NextPage } from "next"
import { useEffect } from "react"

import { Container } from "@/components/Container"
import { useGlobalContext } from "@/context/state"

const Settings: NextPage = () => {
  const { setCurrentPage } = useGlobalContext()
  const openConfigFolder = async () => {
    await invoke('show_in_folder', {
      path: "/Users/jaraddelorenzo/Library/Application Support",
    })
  }

  useEffect(() => {
    setCurrentPage("Settings")
  }, [setCurrentPage])

  return (
    <Container>
      <h1 className="m-0 self-center text-center text-6xl">Settings</h1>
      <button onClick={openConfigFolder}>Open Config Folder</button>
    </Container>
  )
}

export default Settings
