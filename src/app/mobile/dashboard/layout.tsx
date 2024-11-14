"use client";
import BottomNavItem from "@/components/Navigation/BottomNavItem";
import View from "@/components/View";
import {
  BellIcon,
  Cog6ToothIcon,
  HomeIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import {
  BellIcon as SolidBellIcon,
  Cog6ToothIcon as SolidCog6ToothIcon,
  HomeIcon as SolidHomeIcon,
  UserIcon as SolidUserIcon,
} from "@heroicons/react/24/solid";

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
    icon: <BellIcon />,
    alternateIcon: <SolidBellIcon />,
    label: "notification",
    path: "notification",
  },
  {
    icon: <UserIcon />,
    alternateIcon: <SolidUserIcon />,
    label: "profile",
    path: "profile",
  },

  {
    icon: <Cog6ToothIcon />,
    alternateIcon: <SolidCog6ToothIcon />,
    label: "settings",
    path: "settings",
  },
];
export default function MobileAppDashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

  return (
    <View className="pt-4 pb-12 px-6 min-h-screen">
      {children}

      <View className="btm-nav z-50 bg-white rounded-t-xl text-gray-500 py-4">
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
