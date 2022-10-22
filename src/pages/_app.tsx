import "@/styles/globals.scss"

import type { AppProps } from "next/app"
import { GlobalContextWrapper } from "@/context/state";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <GlobalContextWrapper>
      <Component {...pageProps} />
    </GlobalContextWrapper>
  )
}

export default MyApp
