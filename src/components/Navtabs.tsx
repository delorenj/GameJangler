import { Tab } from "@headlessui/react"

export const Navtabs = () => {
  return (
    <Tab.Group className="flex flex-col justify-evenly sm:flex-row">
      <Tab.List>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          Dashboard
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          Apps
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          Saves
        </Tab>
        <Tab className="mt-3 border-b-2 pb-3 font-bold outline-0 transition-all ui-selected:border-b-blue-700 ui-selected:text-black ui-not-selected:border-transparent ui-not-selected:text-gray-400">
          Settings
        </Tab>
      </Tab.List>
    </Tab.Group>
  )
}
