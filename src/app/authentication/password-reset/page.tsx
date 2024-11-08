"use client";
import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { Form, FormProps, Input } from "antd";
import { useState } from "react";

type FormFieldTypes = {
  email?: string;
  password?: string;
};

export default function Page() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values });
  };

  return (
    <View className="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]">
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
          <View className="text-center mb-6">
            <Heading className="font-semibold">Password reset</Heading>
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

          <Button
            href="/authentication/confirm-otp"
            className=" bg-app-600 text-center w-full py-2 text-white"
          >
            Continue
          </Button>
        </Form>
        <SmallText className="text-center mt-4">
          Remember password?{" "}<a className="text-app-600 ml-2" href="/">login</a>
        </SmallText>
      </View>
    </View>
  );
}
