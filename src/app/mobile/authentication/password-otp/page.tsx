"use client";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { ArrowLongLeftIcon } from "@heroicons/react/24/solid";
import { Form, FormProps, GetProps, Input } from "antd";
import Link from "next/link";
import { useRouter } from "next/navigation";

type FormFieldTypes = {
  email?: string;
  password?: string;
};

type OTPProps = GetProps<typeof Input.OTP>;

export default function ForgottenPassword() {
  const [form] = Form.useForm();
  const router = useRouter();
  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
 
   
  };

  const onChange: OTPProps["onChange"] = (text) => {
    console.log("onChange:", text);
  };

  const sharedProps: OTPProps = {
    onChange,
    size: "large",
    className:"py-4"
  };

  return (
    <>
      <Form
        initialValues={{ remember: true }}
        onFinish={submit_form}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 gap-y-2 py-6"
        form={form}
      >
        <View className="mb-14 flex justify-between items-center">
          <Link href="/mobile/authentication/forgotten-password">
            <ArrowLongLeftIcon className="w-6 h-6" />
          </Link>
        </View>

        <View className="mb-6">
          <Heading className="font-bold">Confirm OTP</Heading>
          <Text className="leading-1 mt-2 text-sm">
           If an account exist for <span className="font-semibold">test@user.com,</span> you will receive a password reset token
          </Text>
        </View>

        <Form.Item<FormFieldTypes> label="" name="email">
          <Input.OTP
            {...sharedProps}
            size="large"
            type="number"
          
          />
        </Form.Item>

        <button
          onClick={() => router.push("/mobile/authentication/password-otp")}
          className="w-full rounded-lg py-4  bg-app-600 font-medium text-white "
        >
          Continue
        </button>
      </Form>
      <Link href="/mobile/" className="text-app text-sm font-semibold">
        Didn&apos;t get an OTP? Request new
      </Link>
    </>
  );
}
