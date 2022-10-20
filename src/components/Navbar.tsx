import { Tab } from "@headlessui/react"

import { Navhamburger } from "@/components/Navhamburger"
import { Navlogo } from "@/components/Navlogo"
import { Navtabs } from "@/components/Navtabs"

export const Navbar = () => {
  return (
    <nav className="py-2.5">
      <div className="container flex flex-wrap bg-green-100 px-4 ">
        <div className="flex-1 bg-red-100 sm:bg-red-400">
          <Navlogo />
        </div>
        <div className="flex-1 bg-amber-100">
          <Tab.Group>
            <Tab.List className="flex">
              <Tab>Dashboard</Tab>
              <Tab>Apps</Tab>
              <Tab>Saves</Tab>
              <Tab>Settings</Tab>
            </Tab.List>
          </Tab.Group>
        </div>
        <div className="flex-1 bg-blue-100 text-right">
          <Navhamburger />
        </div>
      </div>
    </nav>
  )
}
