"use client";
import { AppstoreOutlined } from "@ant-design/icons";
import {
  WalletIcon,
  Cog6ToothIcon,
  UserIcon,
  QueueListIcon,
  BellAlertIcon,
} from "@heroicons/react/24/outline";
import { ItemType, MenuItemType } from "antd/es/menu/interface";
import Link from "next/link";

export const routes: ItemType<MenuItemType>[] = [
  {
    icon: <AppstoreOutlined className="w-6 h-6" style={{ fontSize: "24px" }} />,
    label: <Link href={"/dashboard"}>Dashboard</Link>,
    key: 1,
  },
  {
    icon: <WalletIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/vaults"}>Vaults</Link>,
    key: 2,
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/settings"}>Settings</Link>,
    key: 3,
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/profile"}>Profile</Link>,

    key: 4,
  },
  {
    icon: <BellAlertIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/profile"}>Notification</Link>,

    key: "notification",
  },
  {
    icon: <QueueListIcon className="w-6 h-6" />,
    label: <Link href={"/dashboard/activity"}>Activity</Link>,
    key: 5,
  },
];
