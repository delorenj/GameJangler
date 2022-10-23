import { Tab } from "@headlessui/react"

import { Navhamburger } from "@/components/Navhamburger"
import { Navlogo } from "@/components/Navlogo"
import { Navtabs } from "@/components/Navtabs"

export const Navbar = () => {
  return (
    <nav className="py-2.5">
      <div className="dark:border-b-slate-900 border-b-gray-150 container flex flex-col flex-wrap gap-5 border-b-2 px-4 lg:flex-row lg:gap-0">
        <div className="flex-1 content-center justify-center lg:justify-start">
          <Navlogo />
        </div>
        <div className="grow lg:self-center">
          <Navtabs />
        </div>
        <div className="flex-1 text-right">{/*<Navhamburger />*/}</div>
      </div>
    </nav>
  )
}
