import { Tab } from "@headlessui/react"
import Link from "next/link"
import { useEffect, useState } from "react"

import { linkIndexMap, navLinks, useGlobalContext } from "@/context/state"

export const Navtabs = () => {
  const { currentPage } = useGlobalContext()
  const [selectedIndex, setSelectedIndex] = useState(linkIndexMap[currentPage])
  useEffect(() => {
    setSelectedIndex(
      navLinks
        .map((link) => {
          return link.name
        })
        .indexOf(currentPage),
    )

    console.log(currentPage, selectedIndex)
  }, [currentPage, selectedIndex, setSelectedIndex])

  return (
    <Tab.Group
      className="flex flex-col justify-evenly sm:flex-row"
      selectedIndex={selectedIndex}
    >
      <Tab.List>
        <Tab className="ui-selected:border-b-blue-700 dark:ui-selected:text-white ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400 mt-3 border-b-2 pb-3 font-bold outline-0 transition-all">
          <Link href="/">Dashboard</Link>
        </Tab>
        <Tab className="ui-selected:border-b-blue-700 dark:ui-selected:text-white ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400 mt-3 border-b-2 pb-3 font-bold outline-0 transition-all">
          <Link href="/apps">Apps</Link>
        </Tab>
        <Tab className="ui-selected:border-b-blue-700 dark:ui-selected:text-white ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400 mt-3 border-b-2 pb-3 font-bold outline-0 transition-all">
          <Link href="/saves">Saves</Link>
        </Tab>
        <Tab className="ui-selected:border-b-blue-700 dark:ui-selected:text-white ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400 mt-3 border-b-2 pb-3 font-bold outline-0 transition-all">
          <Link href="/settings">Settings</Link>
        </Tab>
      </Tab.List>
    </Tab.Group>
  )
}
