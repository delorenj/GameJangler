import { SettingsManager } from "tauri-settings"

export type PlatformInstance = {
  id: string
  name: string
  location: string
}

export type ConfigSchema = {
  theme: "light" | "dark"
  platforms: PlatformInstance[]
}

export const init_settings = async () => {
  const sm = new SettingsManager<ConfigSchema>({
    theme: "light",
    platforms: [],
  })
  await sm.initialize().then(() => {
    console.log("Nicely set")
  })
}
