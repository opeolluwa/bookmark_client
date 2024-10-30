import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  context?: string;
  href?: string;
  onClick?: () => void;
}
export default function SmallText({
  children,
  className,
  context,
  href,
  onClick,
}: Props) {
  if (href) {
    return (
      <a
        href={href}
        onClick={() => onClick}
        className={
          "leading-5 text-gray-500 text-sm text-inherit cursor-pointer underline underline-offset-1 " +
          className
        }
      >
        {context || children}
      </a>
    );
  }
  return (
    <p
      onClick={() => onClick}
      className={"leading-5 text-gray-500 text-sm text-inherit " + className}
    >
      {context || children}
    </p>
  );
}
