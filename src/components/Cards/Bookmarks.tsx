import {
  EllipsisVerticalIcon,
  PencilSquareIcon,
  PlusCircleIcon,
  TrashIcon,
} from "@heroicons/react/24/outline";
import { Dropdown, MenuProps } from "antd";
import Card from ".";
import Heading from "../Heading";
import Text from "../Text";
import {parseDate} from "./utils";

interface Props {
  name: string;
  description: string;
  date: string;
}
export const items: MenuProps["items"] = [
  {
    key: "add",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <PlusCircleIcon className="w-4 h-4 mr-2" />
        Add entry
      </span>
    ),
  },
  {
    key: "edit",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <PencilSquareIcon className="w-4 h-4 mr-2" />
        Edit
      </span>
    ),
  },

  {
    key: "delete",
    danger: true,
    label: (
      <span className="flex gap-x-[2px] items-center">
        <TrashIcon className="w-4 h-4 mr-2" />
        Delete
      </span>
    ),
  },
];
export default function Bookmark({ name, date, description }: Props) {
  return (
    <Card className="border-[2px] my-3 border-gray-100 flex justify-between items-center cursor-pointer first:mt-0 px-6">
      <div className="flex flex-col justify-center ">
        <Heading className="font-semibold text-gray-600 text-xl leading-relaxed">
          {name}
        </Heading>
        <Text className="leading-1">{description}</Text>
      </div>
      <Text>{parseDate(date)}</Text>
      <Dropdown menu={{ items }}>
        <EllipsisVerticalIcon className="w-6 h-6 cursor-pointer" />
      </Dropdown>
    </Card>
  );
}
