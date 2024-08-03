import {
    CheckCircleIcon,
    Cog6ToothIcon,
    HomeIcon,
    InformationCircleIcon,
    MinusCircleIcon,
    WalletIcon,
} from "@heroicons/react/24/outline";

export interface Route {
  icon: React.ReactNode; // the route icon
  name: string; // the route name
  path: string;
}

export const PRIMARY_ROUTES: Route[] = [
  {
    icon: <HomeIcon className="w-6 h-6" />,
    name: "Home",
    path: "/",
  },
  {
    icon: <WalletIcon className="w-6 h-6" />,
    name: "Bucket",
    path: "/bucket",
  },
  {
    icon: <CheckCircleIcon className="w-6 h-6" />,
    name: "Tasks",
    path: "/tasks",
  },
  {
    icon: <Cog6ToothIcon className="w-6 h-6" />,
    name: "Settings",
    path: "/settings",
  },
];




export const SECONDARY_ROUTES: Route[] = [
  {
    icon: <InformationCircleIcon className="w-6 h-6" />,
    name: "Help & Information",
    path: "/help",
  },

  {
    icon: <MinusCircleIcon className="w-6 h-6" />,
    name: "Logout",
    path: "/",
  },
];
