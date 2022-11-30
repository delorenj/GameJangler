import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

export interface PlatformInstance {
  id: string
  name: string
  location: string
}

export interface SettingsSchema {
  platforms: PlatformInstance[]
}

export const useSettings = () => {
  const [settings, setSettings] = useState<SettingsSchema>()
  useEffect(() => {
    const fetchSettings = async () => {
      return await invoke<SettingsSchema>("load_settings")
    }

    fetchSettings()
      .then((settings) => {
        setSettings(settings)
      })
      .catch(console.error)
  }, [])

  return { settings, setSettings }
}
