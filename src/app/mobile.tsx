"use client";
import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input } from "antd";
import { LoginData } from "../../tauri/bindings/LoginData";
import { FormFieldTypes } from "./page";


export default function MobileAppEntry() {
  const [form] = Form.useForm();
  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_in", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };
  return (
    <View className=" pt-4 px-6">
      <Form
        initialValues={{ remember: true }}
        onFinish={submit_form}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 bg-white py-6 "
        form={form}
      >
        <View className="mb-12">
          <Heading className="font-semibold">Welcome back!</Heading>
          <Text className="leading-1">Sign in to continue</Text>
        </View>
        <Form.Item<FormFieldTypes>
          label="Email"
          name="email"
          rules={[{ required: true, message: "email is required!" }]}
        >
          <Input
            autoFocus
            type="email"
            name="email"
            className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 lowercase"
          />
        </Form.Item>
        <Form.Item<FormFieldTypes>
          label="Password"
          name="password"
          rules={[{ required: true, message: "password is required!" }]}
        >
          <Input.Password
            type="password"
            name="password"
            className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 placeholder:pb-2 px-2 "
            iconRender={(visible) =>
              visible ? <EyeTwoTone /> : <EyeInvisibleOutlined />
            }
          />
        </Form.Item>

        <Button
          href="/dashboard"
          className=" bg-app-600 text-center w-full py-2 text-white"
        >
          Sign in
        </Button>
        <SmallText
          href="/authentication/password-reset"
          className="text-gray-500 font-medium no-underline text-sm mt-4"
        >
          Forgotten password?
        </SmallText>
      </Form>
    </View>
  );
}
