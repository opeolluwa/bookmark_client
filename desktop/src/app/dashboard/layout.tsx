"use client";
import { routes } from "@/components/Navigation";
import { Layout, Menu, theme } from "antd";
import React, { useState } from "react";
const { Content, Sider } = Layout;

interface Props {
  children: React.ReactNode;
}

export default function DashboardLayout({ children }: Props) {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();
  const [collapsed, setCollapsed] = useState(true);

  return (
    <Layout className="h-screen overflow-hidden">
      <Layout>
        <Sider
          width={250}
          style={{ background: colorBgContainer }}
          className="relative bg-gray-50 overflow-hidden "
          collapsible
          collapsed={collapsed}
          onCollapse={(value) => setCollapsed(value)}
        >
          <Menu
            theme="dark"
            defaultSelectedKeys={["dashboard"]}
            mode="inline"
            items={routes}
            className=""
            style={{ height: "100%", borderRight: 0 }}
          />
        </Sider>
        <Layout style={{ padding: "24px" }}>
          <Content
            style={{
              padding: 24,
              margin: 0,
              minHeight: 300,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
              
            }}
            className=""
          >
            {children}
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}
