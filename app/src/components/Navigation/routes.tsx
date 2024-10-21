import { AppstoreOutlined } from "@ant-design/icons";
import {
  ArrowLeftEndOnRectangleIcon,
  Cog6ToothIcon,
  InformationCircleIcon,
  MinusCircleIcon,
  QueueListIcon,
  UserCircleIcon,
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
    name: "Bucket",
    path: "/bucket",
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    name: "Settings",
    path: "/settings",
  },
  {
    icon: <UserIcon className="w-6 h-6" />,
    name: "Profile",
    path: "/help",
  },
  {
    icon: <QueueListIcon className="w-6 h-6" />,
    name: "Activity",
    path: "/help",
  },
];

export const SECONDARY_ROUTES: Route[] = [
  {
    icon: <ArrowLeftEndOnRectangleIcon className="w-6 h-6" />,
    name: "Logout",
    path: "/",
  },
];
