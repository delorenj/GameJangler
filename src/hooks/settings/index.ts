import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

interface PlatformInstance {
  id: string
  name: string
  location: string
}

interface SettingsSchema {
  platforms: PlatformInstance[]
}

export const useSettings = () => {
  const [settings, setSettings] = useState<SettingsSchema>()
  console.log("useSettings init")
  useEffect(() => {
    const fetchSettings = async () => {
      return await invoke<string>("load_settings")
    }

    fetchSettings()
      .then((settings: string | SettingsSchema) => {
        setSettings(settings)
      })
      .catch(console.error)
  }, [])

  return { settings, setSettings }
}
