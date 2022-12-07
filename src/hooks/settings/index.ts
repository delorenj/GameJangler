import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

export enum Platform {
  STEAM = "STEAM",
  EPIC = "EPIC",
}

export enum OS {
  WINDOWS = "windows",
  DARWIN = "darwin",
  LINUX = "linux",
}

export interface PlatformInstance {
  id: string
  platform: Platform
  location: string
}

export interface SettingsSchema {
  os: OS
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
