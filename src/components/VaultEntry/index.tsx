import Card from "../Card";
import React from "react";
import Heading from "../Heading";
import Text from "../Text";
import {
  EllipsisVerticalIcon,
  PencilSquareIcon,
  TrashIcon,
} from "@heroicons/react/24/outline";
import { UserOutlined, AntDesignOutlined } from "@ant-design/icons";
import { Avatar, Tooltip, Dropdown, MenuProps } from "antd";

const items: MenuProps["items"] = [
  {
    key: "1",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <PencilSquareIcon className="w-4 h-4" /> Edit
      </span>
    ),
  },
  {
    key: "2",
    danger: true,
    label: (
      <span className="flex gap-x-[2px] items-center">
        <TrashIcon className="w-4 h-4" /> Remove
      </span>
    ),
  },
];

export default function Component() {
  return (
    <Card className="border-[2px] my-3 border-gray-100 flex justify-between items-center">
      <div className=" flex flex-col justify-center items-center border-r border-gray-200 px-4">
        <Text>Wed</Text>
        <Heading>28</Heading>
      </div>
      <div className="flex flex-col justify-center items-center">
        <Text>Lorem ipsum dolor, sit amet consectetur.</Text>
        <Avatar.Group className="mt-3" size={24}>
          <Avatar src="https://api.dicebear.com/7.x/miniavs/svg?seed=1" />
          <a href="https://ant.design">
            <Avatar style={{ backgroundColor: "#f56a00" }}>K</Avatar>
          </a>
          <Tooltip title="Ant User" placement="top">
            <Avatar
              style={{ backgroundColor: "#87d068" }}
              icon={<UserOutlined />}
            />
          </Tooltip>
          <Avatar
            style={{ backgroundColor: "#1677ff" }}
            icon={<AntDesignOutlined />}
          />
        </Avatar.Group>
      </div>
      <Dropdown menu={{ items }}>
        <EllipsisVerticalIcon className="w-6 h-6 cursor-pointer" />
      </Dropdown>
    </Card>
  );
}
