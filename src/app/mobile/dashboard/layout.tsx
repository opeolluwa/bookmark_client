"use client";
import BottomNavItem from "@/components/Navigation/BottomNavItem";
import View from "@/components/View";
import { StarFilled } from "@ant-design/icons";
import {
  BellIcon,
  Cog6ToothIcon,
  HomeIcon,
  MagnifyingGlassIcon,
  StarIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import {
  BellIcon as SolidBellIcon,
  Cog6ToothIcon as SolidCog6ToothIcon,
  HomeIcon as SolidHomeIcon,
  UserIcon as SolidUserIcon,
  MagnifyingGlassIcon as SolidMagnifyingGlassIcon,
} from "@heroicons/react/24/solid";
// import Viewport from "@/components/Viewport"
import React, { ReactNode } from "react";

export interface ApplicationRoute {
  label: string;
  path?: string;
  icon?: ReactNode;
  alternateIcon?: ReactNode;
  action?: () => void;
}
const bottomNavigation: ApplicationRoute[] = [
  {
    label: "home",
    path: "/",
    icon: <HomeIcon />,
    alternateIcon: <SolidHomeIcon />,
  },
  {
    label: "Search",
    icon: <MagnifyingGlassIcon />,
    alternateIcon: <SolidMagnifyingGlassIcon />,
  },
  {
    icon: <StarIcon />,
    alternateIcon: <StarFilled />,
    label: "favorites",
    path: "favorites",
  },
  {
    icon: <Cog6ToothIcon />,
    alternateIcon: <SolidCog6ToothIcon />,
    label: "settings",
    path: "settings",
  },
  {
    icon: <UserIcon />,
    alternateIcon: <SolidUserIcon />,
    label: "profile",
    path: "profile",
  },
];
export default function MobileAppDashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
      <View className="pt-4 pb-12 px-6 h-screen overflow-y-auto ">
        {children}

        <View className="btm-nav z-50 bg-app-50/70 fixed rounded-t-3xl text-gray-500 py-4">
          {bottomNavigation.map((route) => {
            const { icon, alternateIcon, label, path: slug } = route;
            return (
              <BottomNavItem
                label={label}
                alternateIcon={alternateIcon}
                icon={icon}
                path={slug}
                key={label}
              />
            );
          })}
        </View>
      </View>
  );
}
