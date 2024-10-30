import React, { Children } from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
}
export default function Heading({ children, className,}: Props) {
  return (
    <h2 className={"font-semibold text-2xl " + className}>
      { children}
    </h2>
  );
}
