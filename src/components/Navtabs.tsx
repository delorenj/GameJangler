import { Tab } from "@headlessui/react"

export const Navtabs = () => {
  return (
    <Tab.Group className="flex flex-col justify-evenly sm:flex-row">
      <Tab.List>
        <Tab>Dashboard</Tab>
        <Tab>Apps</Tab>
        <Tab>Saves</Tab>
        <Tab>Settings</Tab>
      </Tab.List>
    </Tab.Group>
  )
}
