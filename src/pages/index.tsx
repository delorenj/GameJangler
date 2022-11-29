import { invoke } from "@tauri-apps/api/tauri"
import type { NextPage } from "next"
import { useRouter } from "next/router"
import { useEffect } from "react"
import { AppCarousel } from "@/components/AppCarousel"
import { Container } from "@/components/Container"
import { SaveCarousel } from "@/components/SaveCarousel"
import { StorageCarousel } from "@/components/StorageCarousel"
import { useGlobalContext } from "@/context/state"
import { useSettings } from "@/hooks/settings"

const Home: NextPage = () => {
  const router = useRouter()
  const { setCurrentPage } = useGlobalContext()
  const { settings } = useSettings()

  useEffect(() => {
    setCurrentPage("Home")
  }, [setCurrentPage])

  useEffect(() => {
    if (!settings) return
    const redirect = async () => {
      return await router.push("/apps")
    }
    if (!settings.platforms) {
      redirect().catch(console.error)
    }
  }, [router, settings])

  return (
    <Container>
      <div className="flex w-full">
        <ul className="flex w-1/2 flex-1 flex-col gap-20">
          <li>
            <SaveCarousel />
          </li>
          <li>
            <AppCarousel />
          </li>
          <li>
            <StorageCarousel />
          </li>
        </ul>
      </div>
    </Container>
  )
}

export default Home
