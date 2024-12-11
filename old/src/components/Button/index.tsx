interface Props {
  children?: React.ReactNode;
  className?: string;
  href?: string;
  onClick?: () => void;
  type?: "submit" | "reset" | "button";
  disabled?: boolean;
}

export default function Button({
  children,
  className,
  onClick,
  href,
  type,
  disabled,
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
        className={
          "rounded px-4 py-3 block w-full disabled:text-app-200 " + className
        }
        onClick={onClick}
        type={type}
        disabled={disabled}
      >
        {children}
      </button>
    );
  }
}
