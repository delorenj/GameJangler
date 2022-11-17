import "@/styles/globals.scss"

import type { AppProps } from "next/app"

import { GlobalContextWrapper } from "@/context/state"
import { ThemeContextWrapper } from "@/context/theme"

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <GlobalContextWrapper>
      <ThemeContextWrapper>
        <Component {...pageProps} />
      </ThemeContextWrapper>
    </GlobalContextWrapper>
  )
}

export default MyApp
