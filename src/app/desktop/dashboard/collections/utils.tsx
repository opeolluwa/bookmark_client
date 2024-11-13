import {
  EllipsisVerticalIcon,
  PencilSquareIcon,
  PlusCircleIcon,
  TrashIcon,
} from "@heroicons/react/24/outline";
import {
  Dropdown,
  GetProp,
  Input,
  MenuProps,
  Space,
  Table,
  TableColumnsType,
  TableProps,
} from "antd";
import { SorterResult } from "antd/es/table/interface";
import { Vault } from "vault_grpc_bindings/bindings";

export type { CommandResponse } from "../../../../../tauri/bindings/CommandResponse";

export interface TableParams {
  pagination?: TablePaginationConfig;
  sortField?: SorterResult<any>["field"];
  sortOrder?: SorterResult<any>["order"];
  filters?: Parameters<GetProp<TableProps, "onChange">>[1];
}

export const { TextArea } = Input;
export const { Column } = Table;

const parseDate = (dateString: string) => {
  return new Date(dateString.split("+")[0].trim()).toLocaleDateString("en-US", {
    year: "numeric",
    weekday: "short",
    day: "numeric",
    month: "short",
    timeZone: "UTC",
    minute: "numeric",
    hour: "numeric",
  });
};

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
    key: "set default",
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
        <Space />
        Delete
      </span>
    ),
  },
];

export interface DataType extends Vault {
  vault_id: string;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
}

export const columns: TableColumnsType<DataType> = [
  {
    title: "Name",
    dataIndex: "name",
    key: "name",
  },
  {
    title: "Description",
    dataIndex: "description",
    key: "description",
  },
  {
    title: "Created At",
    dataIndex: "created_at",
    key: "created_at",
    render: (dateString: string) => parseDate(dateString),
  },
  {
    title: "Updated at",
    key: "updated_at",
    dataIndex: "updated_at",
    render: (dateString: string) => parseDate(dateString),
  },
  {
    title: "Action",
    key: "action",
    dataIndex: "action",
    render: () => (
      <Space size="middle">
        <Dropdown menu={{ items }}>
          <EllipsisVerticalIcon className="w-6 h-6 cursor-pointer" />
        </Dropdown>
      </Space>
    ),
  },
];
export type FormFieldTypes = {
  name?: string;
  description?: string;
};

export type TablePaginationConfig = Exclude<
  GetProp<TableProps, "pagination">,
  boolean
>;
