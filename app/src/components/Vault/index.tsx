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
      <div className="flex flex-col justify-center ">
        <Heading className="font-semibold text-gray-600 text-xl">Lorem ipsum dolor, sit amet consectetur.</Heading>
        <Text>Lorem ipsum dolor, sit amet consectetur.</Text>
      </div>
      <Text>Mon Oct 24, 1999</Text>
      <Dropdown menu={{ items }}>
        <EllipsisVerticalIcon className="w-6 h-6 cursor-pointer" />
      </Dropdown>
    </Card>
  );
}
