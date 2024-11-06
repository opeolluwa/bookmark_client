"use client";
import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { Checkbox, Form, FormProps, Input } from "antd";
import { EyeTwoTone, EyeInvisibleOutlined } from "@ant-design/icons";

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

  };

  return (
    <View className="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]">
      <View className=" py-8 pb-12 px-6 w-[40%] ">
        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 mt-10 flex flex-col rounded-lg shadow-gray-300 bg-white px-8 py-6 "
          form={form}
        >
          <View className="text-center mb-6">
            <Heading className="font-semibold">Create your account</Heading>
          </View>

          <Form.Item<FormFieldTypes>
            label="First name"
            name="firstName"
            rules={[{ required: true, message: "first name is required " }]}
          >
            <Input
              autoFocus
              type="text"
              name="firstName"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Last name"
            name="lastName"
            rules={[{ required: true, message: "last name is required" }]}
          >
            <Input
              autoFocus
              type="text"
              name="title"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Email"
            name="email"
            rules={[{ required: true, message: "invalid email" }]}
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
            rules={[{ required: true, message: "password is required" }]}
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
          <View className="flex gap-2 my-2 -mt-2">
            <Checkbox /> <SmallText>I agree to the terms and privacy</SmallText>
          </View>
          <Button
            onClick={() => submitForm}
            className=" bg-app-600 text-center w-full py-2 text-white"
          >
            Sign up
          </Button>
        </Form>
        <SmallText className="text-center">
          Already have an account?{" "}
          <a className="text-app-600 text-center" href="/">
            Sign in
          </a>
        </SmallText>
      </View>
    </View>
  );
}
