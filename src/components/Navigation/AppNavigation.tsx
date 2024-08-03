
"use client";

import { AppLogo } from "../App/inedx";
import NavigationTab from "./NavItems";
import { PRIMARY_ROUTES, SECONDARY_ROUTES } from "./routes";
export default function AppNavigation() {
  return (
    <nav
      className=" sm:col-span-3 lg:col-span-3 bg-[rgba(249,250,254,255)]   pl-4   text-gray-600  pt-10 border-r-2 border-gray-200"
      style={{
        height: "calc(100vh-200px)",
        overflowY: "hidden",
        position: "relative",
      }}
    >
      <AppLogo/>
      <div className="flex flex-col px-2 mb-8">
        {PRIMARY_ROUTES.map((route, index) => (
          <NavigationTab
            key={index}
            icon={route.icon}
            name={route.name}
            path={route.path}
          />
        ))}
      </div>
      <div className="flex flex-col px-2 absolute bottom-[40px] w-100 ">
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
