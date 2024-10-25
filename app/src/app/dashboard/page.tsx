"use client";
import Card from "@/components/Card";
import { Greeting } from "@/components/Greetings";
import View from "@/components/View";
import VaultEntry from "@/lib/VaultEntry";
import VaultEntryComponent from "@/components/VaultEntry";
import { Pagination } from "antd";
import DefaultVault from "@/store/vault";
import {
  AdjustmentsHorizontalIcon,
  ChevronDownIcon,
} from "@heroicons/react/24/outline";
import { PlusIcon } from "@heroicons/react/24/solid";
import type { FormProps } from "antd";
import {
  Button,
  FloatButton,
  Form,
  Input,
  Modal,
  notification,
  Segmented,
} from "antd";
import { SearchProps } from "antd/es/input";
import { useEffect, useState } from "react";
import { TagsInput } from "react-tag-input-component";
const { TextArea, Search } = Input;

type FormFieldTypes = {
  title?: string;
  description?: string;
};

export default function Home() {
  const [greeting, setGreeting] = useState("");
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [keywords, setKeywords] = useState<Array<string>>([]);
  const [vaultEntries, setVaultEntries] = useState<Array<VaultEntry>>(
    DefaultVault.content
  );
  const [api, contextHolder] = notification.useNotification();
  const [form] = Form.useForm();
  const onSearch: SearchProps["onSearch"] = (value, _e, info) =>
    console.log(info?.source, value);

  const openNotification = () =>
    api.open({
      message: "entry saved!",
      description:
        "I will never close automatically. This is a purposely very very long description that has many many characters and words.",
      duration: 10,
      placement: "bottomRight",
    });

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values, keywords });
    const vault_entry = new VaultEntry(
      String(values.title),
      String(values.description),
      keywords
    );

    DefaultVault.add_entry(vault_entry);
    setVaultEntries(DefaultVault.content);
    console.log(JSON.stringify(DefaultVault.content));
    setIsModalOpen(false);
    form.resetFields();
    setKeywords([]);
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
    let text = new Greeting().msg;
    const punctuation = /[.,!?]$/;

    if (punctuation.test(text)) {
      text = text; // If it ends with punctuation, return as-is
    } else {
      text = text + ","; // If it doesn't, add a comma
    }
    setGreeting(text);
  }, []);

  return (
    <>
      <View className="">{greeting} Adeoye</View>
      <View className="my-6">
        <View className="flex justify-between items-center">
          <Segmented<string>
            options={["This week", "Last week", "Older"]}
            onChange={(value) => {
              console.log(value); // string
            }}
            className="mb-2"
          />
          <AdjustmentsHorizontalIcon className="text-gray-400 h-6 w-6 cursor-pointer" />
        </View>
        <VaultEntryComponent />
        <VaultEntryComponent />
        <VaultEntryComponent />
        <VaultEntryComponent />
        <VaultEntryComponent />
      </View>
      <Pagination align="center" defaultCurrent={1} total={50} />
      <View className=" h-screen ">
        <Modal
          title="Save to vault"
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
            </Button>
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
              name="description"
              rules={[
                { required: true, message: "Please input the description!" },
              ]}
            >
              <TextArea
                showCount
                maxLength={100}
                name="description"
                style={{ height: 120, resize: "none" }}
              />
            </Form.Item>
            <View>
              <label className="mb-2 block text-sm">Keywords</label>
              <TagsInput
                value={keywords}
                onChange={setKeywords}
                name="keywords"
                separators={["Enter", "Space"]}
                classNames={{
                  input:
                    "w-full rounded p-2 focus:border-app-500 focus:outline-none border-2 bg-white border-gray-300 block w-full w-100",
                }}
                placeHolder="add tags"
              />
            </View>
          </Form>
        </Modal>

        <View>
          {vaultEntries.map((item, key) => (
            <Card
              key={key}
              className="my-4 first: mt-6 cursor-pointer flex justify-between items-center bg-gray-50"
            >
              {item.title} <ChevronDownIcon className="w-6 h-6" />{" "}
            </Card>
          ))}
        </View>
        <FloatButton
          shape="circle"
          type="primary"
          className="mr-3 -mb-3"
          style={{ insetInlineEnd: 24 }}
          icon={<PlusIcon className="w-4 h-4" />}
          onClick={showModal}
        />
      </View>
    </>
  );
}
