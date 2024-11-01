"use client";
// import Button from "@/components/Button";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Button, Divider, Form, Input, Switch } from "antd";
import { FormProps } from "antd/es/form";
import { User } from "../../../../tauri/bindings/User";
type FormFieldTypes = {
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  acceptEULA?: boolean;
};

export default function Page() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: User = {
      firstName: values.firstName?.trim(),
      lastName: values.lastName?.trim(),
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_up", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };

  return (
    <>
      <Heading>Profile</Heading>
      <Text> Manage your profile settings</Text>
      <Divider />
      <View className="">
        <Heading className="font-normal">Basic info</Heading>
        <Text> Update your bio information</Text>

        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="flex flex-col w-[45%] py-6 "
          form={form}
        >
          <Form.Item<FormFieldTypes> label="First name" name="firstName">
            <Input
              autoFocus
              type="text"
              name="firstName"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes> label="Last name" name="lastName">
            <Input
              autoFocus
              type="text"
              name="title"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes> label="Email" name="email">
            <Input
              autoFocus
              type="email"
              name="email"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes> label="Password" name="password">
            <Input.Password
              autoFocus
              type="password"
              name="password"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 placeholder:pb-2 px-2 "
              iconRender={(visible) =>
                visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />
              }
            />
          </Form.Item>

          <Button
            onClick={() => submitForm}
            className=" bg-app-600 text-center w-full py-6 text-white"
          >
            Update profile
          </Button>
        </Form>
      </View>
      <Divider />
      <View>
        <Heading className="font-normal">Two-factor authentication</Heading>
        <Text> Add an extra layer of security to your data</Text>

        <View className="my-3 flex gap-3 ">
          <Text>Enable 2FA</Text> <Switch />
        </View>
      </View>
    </>
  );
}
