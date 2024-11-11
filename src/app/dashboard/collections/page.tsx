"use client";
import Bookmark from "@/components/Cards/Bookmarks";
import Heading from "@/components/Heading";
import View from "@/components/View";
import { invoke } from "@tauri-apps/api/core";
import type { FormProps } from "antd";
import { Button, Form, Input, Modal } from "antd";
import { useEffect, useState } from "react";
import type {
  ListVaultsRequest,
  ListVaultsResponse,
} from "vault_grpc_bindings/bindings";
import { CommandResponse, FormFieldTypes, TextArea } from "./utils";

export default function Home() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [vaults, setVaults] = useState<ListVaultsResponse>();
  const [form] = Form.useForm();

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values });
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
    };

    fetchData();
  });

  return (
    <>
      <View className="my-6 relative">
        <View className="flex justify-between items-center">
          <Heading>Collections</Heading>

          <Button
            className="w-fit hover:bg-black bg-app-600 shadow text-white border-none"
            onClick={showModal}
          >
            Create new
          </Button>
        </View>
      </View>

      <View className="">
        {vaults?.vaults.map((entry) => (
          <Bookmark
            name={entry.name}
            description={entry.description}
            date={entry.created_at}
            key={entry.vault_id}
          />
        ))}
      </View>

      <View className="h-screen">
        <Modal
          title="New Collection"
          open={isModalOpen}
          onOk={handleOk}
          centered
          onCancel={handleCancel}
          afterClose={() => submitForm}
          footer={[
            <Button key="back" onClick={handleCancel}>
              Cancel
            </Button>,
            <Button
              key="submit"
              type="primary"
              htmlType="submit"
              onClick={handleOk}
            >
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
