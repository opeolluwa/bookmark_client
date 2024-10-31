"use client";
// import Button from "@/components/Button";
import Heading from "@/components/Heading";
import View from "@/components/View";
import { EyeInvisibleOutlined, EyeTwoTone } from "@ant-design/icons";
import { invoke } from "@tauri-apps/api/core";
import { Avatar, Form, Input , Button} from "antd";
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
      <Heading className="mb-3">Profile</Heading>
      <View className="flex items-center gap-3">
        <Avatar size={120} src="/avatar.png" />{" "}
        <Button className="bg-app-600 text-white w-fit cursor-pointer">
          Change picture{" "}
        </Button>
        <Button className="bg-transparent py-4 text-red-600 border-[1px] border-red-600 cursor-pointer  w-fit">
          Delete picture{" "}
        </Button>
      </View>
      <View>
        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 mt-10 flex flex-col w-[45%] py-6 "
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
            className=" bg-app-600 text-center w-full py-2 text-white"
          >
            Update profile
          </Button>
        </Form>
      </View>
    </>
  );
}
