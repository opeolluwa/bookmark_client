"use client";
import AppNavigation from "@/components/Navigation/AppNavigation";
import { Breadcrumb, Layout, theme } from "antd";
import React, { useState } from "react";
const { Header, Content, Sider } = Layout;

interface Props {
  children: React.ReactNode;
}

export default function DashboardLayout({ children }: Props) {
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();
  const [collapsed, setCollapsed] = useState(false);

  return (
    <Layout className="h-screen">
      <Layout>
        <Sider
          width={250}
          style={{ background: colorBgContainer }}
          className="h-screen bg-gray-50 "
          collapsible
          collapsed={collapsed}
          onCollapse={(value) => setCollapsed(value)}
        >
          <AppNavigation />
        </Sider>
        <Layout style={{ padding: "0 24px 24px" }}>
          <Breadcrumb
            items={[
              { title: "Adeoye" },
              { title: "Vaults" },
              { title: "Default" },
            ]}
            style={{ margin: "16px 0" }}
          />
          <Content
            style={{
              padding: 24,
              margin: 0,
              minHeight: 300,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
            }}
          >
            {children}
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
}
