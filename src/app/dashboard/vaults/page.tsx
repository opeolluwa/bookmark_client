"use client";
import Heading from "@/components/Heading";
import View from "@/components/View";
import { PlusIcon } from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/core";
import type { FormProps, TableProps } from "antd";
import { Button, Form, Input, Modal, Table } from "antd";
import { SearchProps } from "antd/es/input";
import { useEffect, useState } from "react";
import type {
  ListVaultsRequest,
  ListVaultsResponse,
} from "vault_grpc_bindings/bindings";
import {
  Column,
  columns,
  CommandResponse,
  DataType,
  FormFieldTypes,
  TableParams,
  TextArea,
} from "./utils";

export default function Home() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [vaults, setVaults] = useState<ListVaultsResponse>();
  const [keywords, setKeywords] = useState<Array<string>>([]);
  const [form] = Form.useForm();
  const [tableParams, setTableParams] = useState<TableParams>({
    pagination: {
      current: 1,
      pageSize: 5,
    },
  });
  const [loading, setLoading] = useState(false);
  const onSearch: SearchProps["onSearch"] = (value, _e, info) =>
    console.log(info?.source, value);

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values, keywords });
    setIsModalOpen(false);
    form.resetFields();
  };

  const submitFormFailed: FormProps<FormFieldTypes>["onFinishFailed"] = (
    errorInfo
  ) => {
    console.log("Failed:", errorInfo);
  };

  const showModal = () => {
    setIsModalOpen(true);
  };
  const handleOk = () => {
    setIsModalOpen(false);
    form.submit();
  };
  const handleCancel = () => {
    setIsModalOpen(false);
    form.resetFields();
  };

  useEffect(() => {
    const fetchData = async () => {
      const payload: ListVaultsRequest = {
        page: 1,
        page_size: 5,
      };
      const response = await invoke("get_all_vaults", { payload }).catch(
        (error) => {
          console.log(error.message);
        }
      );
      let result = response as unknown as CommandResponse<ListVaultsResponse>;
      setVaults(result.body as ListVaultsResponse);
      setTableParams({
        ...tableParams,
        pagination: {
          ...tableParams.pagination,
          total: result.body?.total_count,
        },
      });
    };

    fetchData();
  }, [
    tableParams.pagination?.current,
    tableParams.pagination?.pageSize,
    tableParams?.sortOrder,
    tableParams?.sortField,
    JSON.stringify(tableParams.filters),
  ]);

  const handleTableChange: TableProps<DataType>["onChange"] = (
    pagination,
    filters,
    sorter
  ) => {
    setTableParams({
      pagination,
      filters,
      sortOrder: Array.isArray(sorter) ? undefined : sorter.order,
      sortField: Array.isArray(sorter) ? undefined : sorter.field,
    });

    // `dataSource` is useless since `pageSize` changed
    if (pagination.pageSize !== tableParams.pagination?.pageSize) {
      //  setVaults();
    }
  };

  return (
    <>
      <View className="my-6 relative">
        <View className="flex justify-between items-center">
          <Heading>Vaults</Heading>

          <Button
            className=" w-fit bg-app-600 shadow text-white inline-flex text-sm  px-2"
            onClick={showModal}
          >
            <PlusIcon className="w-6 h-6" /> Create new{" "}
          </Button>
        </View>
      </View>
      <View>
        <Table<DataType>
          className="my-6 cursor-pointer"
          dataSource={vaults?.vaults as unknown as DataType[]}
          columns={columns}
          pagination={{
            ...tableParams.pagination,
            position: ["bottomCenter", "bottomCenter"],
          }}
          loading={loading}
        >
          <Column title="Name" dataIndex="name" key="name" />
          <Column
            title="Description"
            dataIndex="description"
            key="description"
          />
          <Column
            title="Date created"
            dataIndex="created_at"
            key="created_at"
          />
          <Column
            title="Last modified"
            dataIndex="updated_at"
            key="updated_at"
          />

          <Column title="Action" key="action" dataIndex="action" />
        </Table>
      </View>

      <View className=" h-screen ">
        <Modal
          title="New Vault"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          afterClose={() => submitForm}
          footer={[
            <Button key="back" onClick={handleCancel}>
              Cancel
            </Button>,
            <Button key="submit" type="primary" onClick={handleOk}>
              Submit
            </Button>,
          ]}
        >
          <Form
            initialValues={{ remember: true }}
            onFinish={submitForm}
            onFinishFailed={submitFormFailed}
            autoComplete="off"
            name="save-data"
            layout="vertical"
            className="my-4 flex flex-col gap-y-"
            form={form}
          >
            <Form.Item<FormFieldTypes>
              label="Title"
              name="title"
              rules={[{ required: true, message: "Please input the title!" }]}
            >
              <Input
                autoFocus
                type="text"
                name="title"
                className="w-full rounded py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
              />
            </Form.Item>
            <Form.Item<FormFieldTypes>
              label="Description"
              rules={[
                { required: true, message: "Please input the description" },
              ]}
            >
              <TextArea
                showCount
                maxLength={100}
                name="description"
                style={{ height: 120, resize: "none" }}
              />
            </Form.Item>
          </Form>
        </Modal>
      </View>
    </>
  );
}
