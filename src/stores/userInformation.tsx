"use client";

import { createContext, ReactNode, useContext } from "react";
import { ProfileResponse as UserProfile } from "bookmark_grpc_codegen/bindings/ProfileResponse";

const Context = createContext<UserProfile>({} as UserProfile);

export function UserInformationProvider({ children }: { children: ReactNode }) {
  return (
    <Context.Provider value={{} as UserProfile}>{children}</Context.Provider>
  );
}

export function useUserProfileInformation() {
  return useContext(Context);
}
