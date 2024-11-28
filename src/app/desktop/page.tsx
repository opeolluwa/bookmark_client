"use client";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { message } from "@tauri-apps/plugin-dialog";
import { Form, FormProps, Input } from "antd";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { httpClient } from "../axios";
import { LoginRequest as FormFieldTypes } from "bookmark_grpc_codegen/bindings/LoginRequest";
import SubmitButton from "@/components/Button/SubmitButton";
import { loginHandler } from "@/hooks/useIsLoggedIn";

export default function DesktopAppEntry() {
  const [form] = Form.useForm();
  const router = useRouter();
  const [isProcessingForm, setIsProcessingForm] = useState(false);

  const submitForm: FormProps<FormFieldTypes>["onFinish"] = async (values) => {
    setIsProcessingForm(true);
    const payload: FormFieldTypes = {
      email: values.email.trim(),
      password: values.password.trim(),
    };
  };

  return (
    <View className="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full ">
      <View className="col-span-5 py-8 px-6 w-[40%] ">
        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 mt-12 flex flex-col rounded-lg shadow-lg shadow-gray-300 bg-white px-8 py-6 "
          form={form}
        >
          <View className="text-center mb-3">
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
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Password"
            name="password"
            rules={[{ required: true, message: "password is required!" }]}
          >
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

          <Form.Item>
            <SubmitButton
              className=" bg-app-600 btn disabled:bg-app-600 disabled:text-white border-white w-full text-white"
              text="Sign in"
              loadingState={isProcessingForm}
            />
          </Form.Item>

          <SmallText
            href="/authentication/password-reset"
            className="text-right  underline mt-4"
          >
            Forgotten password?
          </SmallText>
        </Form>
        <SmallText className="text-center mt-4">
          Don&apos;t have an account?
          <a className="text-app-600 ml-2" href="/authentication/signup">
            Sign up
          </a>
        </SmallText>
      </View>
    </View>
  );
}
