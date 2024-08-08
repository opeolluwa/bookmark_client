
"use client";

import { AppLogo } from "../App/inedx";
import NavigationTab from "./NavItems";
import { PRIMARY_ROUTES, SECONDARY_ROUTES } from "./routes";
export default function AppNavigation() {
  return (
    <nav
      className=" sm:col-span-3 lg:col-span-3 bg-app-100   px-8   text-gray-600  pt-10 border-r-2 border-gray-100"
      style={{
        height: "calc(100vh-200px)",
        overflowY: "hidden",
        position: "relative",
      }}
    >
      <AppLogo className="border-b-2 border-slat-200 pb-8 "/>
      <div className="flex flex-col px-2 my-8 -mx-5">
        {PRIMARY_ROUTES.map((route, index) => (
          <NavigationTab
            key={index}
            icon={route.icon}
            name={route.name}
            path={route.path}
          />
        ))}
      </div>
      <div className="flex flex-col px-2 absolute bottom-[40px] max-w-full -mx-5  ">
        {SECONDARY_ROUTES.map((route, index) => (
          <NavigationTab
            key={index}
            icon={route.icon}
            name={route.name}
            path={route.path}
          />
        ))}
      </div>
    </nav>
  );
}
