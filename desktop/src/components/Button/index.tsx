interface Props {
  children?: React.ReactNode;
  className?: string;
  href?: string;
  onClick?: () => void;
  type?: "submit" | "reset" | "button";
}

export default function Button({
  children,
  className,
  onClick,
  href,
  type,
}: Props) {
  if (href) {
    return (
      <a
        href={href}
        onClick={onClick}
        className={"rounded px-4 py-3 block cursor-pointer " + className}
      >
        {children}
      </a>
    );
  } else {
    return (
      <button
        className={"rounded px-4 py-3 block w-full " + className}
        onClick={onClick}
        type={type}
      >
        {children}
      </button>
    );
  }
}
