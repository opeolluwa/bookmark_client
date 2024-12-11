export interface SubmitButtonProps {
  loadingState: boolean;
  text: string;
  className?: string;
}

import React from "react";

export default function SubmitButton({
  text,
  loadingState,
  className,
}: SubmitButtonProps) {
  return (
    <button disabled={loadingState} className={className} type="submit">
      {loadingState ? (
        <span className="loading loading-ring loading-sm"></span>
      ) : (
        <span>{text}</span>
      )}
    </button>
  );
}
