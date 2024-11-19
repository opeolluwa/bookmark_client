"use client";
import {
  EllipsisVerticalIcon,
  PencilSquareIcon,
  ShareIcon,
  StarIcon,
  TrashIcon,
} from "@heroicons/react/24/outline";
import { Dropdown, MenuProps } from "antd";
import Card from ".";
import SmallText from "../SmallText";
import Text from "../Text";
import Gravatar from "react-gravatar";
import {
  BookmarkCollection,
  BookmarkCollectionEntries,
} from "vault_grpc_bindings/bindings";
export interface BookmarkProps extends BookmarkCollectionEntries {
  name: string;
  description: string;
  isStarred?: boolean;
}
export const items: MenuProps["items"] = [
  {
    key: "star",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <StarIcon className="w-4 h-4 mr-2" />
        Star
      </span>
    ),
  },
  {
    key: "share",
    label: (
      <span className="flex gap-x-[2px] items-center">
        <ShareIcon className="w-4 h-4 mr-2" />
        Share
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
export default function Bookmark({
  name,
  description,
  isStarred,
}: BookmarkProps) {
  return (
    <Card className="flex gap-x-3 justify-between items-center">
      <div className="flex gap-x-3 items-center">
        <Gravatar
          email={name}
          className="grayscale-0 w-[50px] h-[50px] rounded-lg"
        />
        <div className="flex flex-col">
          <Text className="text-bold ic  flex gap-2 text-black">
            {" "}
            {name}
            {"  "}
            {isStarred && (
              <StarIcon className="w-4 h-4 text-app-secondary-200" />
            )}
          </Text>
          <SmallText className="truncate w-[40vw]">{description}</SmallText>
        </div>
      </div>
      <Dropdown menu={{ items }}>
        <EllipsisVerticalIcon className="w-6 h-6 text-gray-400 justify-self-end" />
      </Dropdown>
    </Card>
  );
}
