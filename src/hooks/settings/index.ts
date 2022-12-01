import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

export enum Platform {
  STEAM = "steam",
  EPIC = "epic",
}

export interface PlatformInstance {
  id: string
  platform: Platform
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
