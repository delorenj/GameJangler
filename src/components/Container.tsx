import { Navbar } from "@/components/Navbar";
import Head from "next/head";
import Image from "next/image"

export const Container = ({children}) => {
  return (
    <div className="flex min-h-screen flex-col">
      <Head>
        <title>Game Jangler</title>
        <meta name="description" content="Gaming cloud save manager" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="flex flex-col justify-center">
        <Navbar />
        <div className="flex h-[800px] justify-center">
          {children}
        </div>
      </main>

      <footer className="flex flex-1 flex-grow-0 items-center justify-center border-t dark:border-slate-700 border-gray-200 py-4">
        <div>
          <a
            href="https://tauri.app/"
            target="_blank"
            rel="noopener noreferrer"
            className="flex flex-grow items-center justify-center p-4 text-slate-700"
          >
            Powered by{" "}
            <span className="ml-2 h-6">
              <Image
                src="/tauri_logo_light.svg"
                alt="Vercel Logo"
                height={24}
                width={78}
              />
            </span>
          </a>
        </div>
      </footer>
    </div>
  );
}
