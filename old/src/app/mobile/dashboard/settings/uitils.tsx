"use client";
import {
  BellAlertIcon,
  LanguageIcon,
  LockClosedIcon,
  MoonIcon,
  SunIcon,
  UserIcon,
} from "@heroicons/react/24/outline";
import { Dropdown, MenuProps, Space } from "antd";
import { ReactNode } from "react";
import { ApplicationRoute } from "../layout";
import { QuestionCircleOutlined } from "@ant-design/icons";

const language: MenuProps["items"] = [
  {
    key: "english",
    label: "english",
    disabled: true,
  },
  {
    key: "french",
    label: "French",
  },
  {
    key: "spanish",
    label: "Spanish",
  },
];

export const accountSettingsRoutes: ApplicationRoute[] = [
  {
    label: "Personal Information",
    icon: <UserIcon className="" />,
    path: "personal-information",
  },
  {
    label: "Password and Security",
    icon: <LockClosedIcon />,
    path: "password-and-security",
  },
  {
    label: "Notification and Preferences",
    icon: <BellAlertIcon className="" />,
    path: "notification-and-preferences",
  },
];

export const settingsTab: {
  title: string | ReactNode;
  routes?: ApplicationRoute[];
  component?: ReactNode;
}[] = [
  {
    title: "account settings",
    routes: accountSettingsRoutes,
  },
  {
    title: (
      <>
        <span>
          Backup Settings <QuestionCircleOutlined className="size-8 ml-1" />
        </span>
      </>
    ),
    component: (
      <>
        <ul className="flex flex-col gap-y-4">
          <li className="flex items-center gap-x-4">
            {" "}
            <input
              type="radio"
              name="radio-1"
              className="radio radio-sm"
              defaultChecked
            />
            Automatic backup{" "}
          </li>
          <li className="flex items-center gap-x-4">
            {" "}
            <input
              type="radio"
              name="radio-1"
              className="radio radio-sm"
            />{" "}
            Manually backup{" "}
          </li>
        </ul>
      </>
    ),
  },
  {
    title: "Language",
    component: (
      <>
        <Dropdown menu={{ items: language }}>
          <a onClick={(e) => e.preventDefault()}>
            <Space>
              <LanguageIcon className="size-5 px-0" /> <span>English</span>
            </Space>
          </a>
        </Dropdown>
      </>
    ),
  },
  {
    title: "color theme",
    component: (
      <label className="flex cursor-pointer gap-2">
        <SunIcon className="size-5" />
        <input
          type="checkbox"
          value="synthwave"
          className="toggle theme-controller"
        />
        <MoonIcon className="size-5" />
      </label>
    ),
  },
];
