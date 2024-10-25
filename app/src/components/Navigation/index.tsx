import { AppstoreOutlined } from "@ant-design/icons";
import {
  WalletIcon,
  Cog6ToothIcon,
  UserIcon,
  QueueListIcon,
} from "@heroicons/react/24/outline";
import { ItemType, MenuItemType } from "antd/es/menu/interface";

export const routes: ItemType<MenuItemType>[] = [
  {
    icon: <AppstoreOutlined className="w-6 h-6" style={{ fontSize: "24px" }} />,
    label: "Dashboard",
    // path: "/dashboard",
    key: 1,
  },
  {
    icon: <WalletIcon className="w-6 h-6" />,
    label: "Vaults",
    // path: "/dashboard/vaults",
    key: 2,
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    label: "Settings",
    // path: "/dashboard/settings",
    key: 3,
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    label: "Profile",
    // path: "/dashboard/profile",
    key: 4,
  },
  {
    icon: <QueueListIcon className="w-6 h-6" />,
    label: "Activity",
    // path: "/help",
    key: 5,
  },
];
