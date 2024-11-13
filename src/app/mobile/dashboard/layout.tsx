"use client"
import View from "@/components/View";
import { BellIcon, Cog6ToothIcon, UserIcon } from "@heroicons/react/24/outline";
import { HomeIcon } from "@heroicons/react/24/solid";
import React, { ReactNode } from "react";

interface Route {
  label: string;
  path?: string;
  icon?: ReactNode;
  action?: () => void;
}
const bottom_navigation: Route[] = [
  {
    label: "home",
    path: "/",
    icon: <HomeIcon className="w-6 h-6" />,
  },
  {
    icon: <BellIcon className="w-6 h-6" />,
    label: "notification",
    path: "notification",
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    label: "profile",
    path: "profile",
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
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
    <View className="py-12 px-6">
      {children}

      <View className="btm-nav py-3">
        {bottom_navigation.map((route) => (
          <a href={`mobile/dashboard/${route.path}`}>
            {route.icon}
            <span className="btm-nav-sm">{route.label}</span>
          </a>
        ))}
      </View>
    </View>
  );
}
