import { AppstoreOutlined } from "@ant-design/icons";
import {
  ArrowLeftEndOnRectangleIcon,
  Cog6ToothIcon,
  QueueListIcon,
  UserIcon,
  WalletIcon,
} from "@heroicons/react/24/outline";

export interface Route {
  icon: React.ReactNode; // the route icon
  name: string; // the route name
  path: string;
}

export const PRIMARY_ROUTES: Route[] = [
  {
    icon: <AppstoreOutlined className="w-6 h-6" style={{ fontSize: "24px" }} />,
    name: "Dashboard",
    path: "/dashboard",
  },
  {
    icon: <WalletIcon className="w-6 h-6" />,
    name: "Vaults",
    path: "/dashboard/vaults",
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    name: "Settings",
    path: "/dashboard/settings",
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    name: "Profile",
    path: "/dashboard/profile",
  },
  {
    icon: <QueueListIcon className="w-6 h-6" />,
    name: "Activity",
    path: "/help",
  },
];
