import type { GetStaticProps, NextPage } from "next"
import Head from "next/head"
import Image from "next/image"
import { useEffect } from "react"
import { getAll } from "tauri-settings"

import { AppCarousel } from "@/components/AppCarousel"
import { Container } from "@/components/Container"
import { Navbar } from "@/components/Navbar"
import { SaveCarousel } from "@/components/SaveCarousel"
import { StorageCarousel } from "@/components/StorageCarousel"
import { ServerState, useGlobalContext } from "@/context/state"
import { ConfigSchema, init_settings, PlatformInstance } from "@/hooks/settings_manager"

interface HomeProps {
  serverState: ServerState
  appSettings: ConfigSchema
}

const Home: NextPage<HomeProps> = (props) => {
  const { setCurrentPage } = useGlobalContext()
  const { serverState, appSettings } = props
  useEffect(() => {
    setCurrentPage("Home")
    console.log(`State received as: ${serverState}`)
  }, [setCurrentPage, serverState, appSettings])

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

export const getStaticProps: GetStaticProps<HomeProps> = async () => {
  const appSettings = await getAll<ConfigSchema>()
  return {
    props: {
      serverState: ServerState.init,
      appSettings,
    },
  }
}

export default Home
