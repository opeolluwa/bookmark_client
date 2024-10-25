"use client";

import { AppLogo } from "../App/AppLogo";
import Heading from "../Heading";
import SmallText from "../SmallText";
import NavigationTab from "./NavItems";
import { PRIMARY_ROUTES } from "./routes";
import user from "@/store/user";

export default function AppNavigation() {
  return (
    <nav
      className=" px-8   text-gray-600  pt-10 border-r-2 border-gray-100"
      style={{
        height: "calc(100vh-200px)",
        overflowY: "hidden",
        position: "relative",
      }}
    >
      <div className="flex gap-x-2 mb-8 hidden">
        <AppLogo className="w-[40px]" />
        <div>
          <Heading className="text-base py-[1px]">Vault</Heading>
          <SmallText>{user.fullName}</SmallText>
        </div>
      </div>
      <div className="flex flex-col px-2 ">
        {PRIMARY_ROUTES.map((route, index) => (
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
