"use client";
import { createContext, useContext, useState } from "react";

type User = { id: string };

const UserContext = createContext<{
	user: User;
	setUser: (u: User) => void;
}>({
	user: { id: "" },
	setUser: () => {},
});

export const UserProvider = ({ children }: { children: React.ReactNode }) => {
	const [user, setUser] = useState<User>({ id: "" });
	return (
		<UserContext.Provider value={{ user, setUser }}>
			{children}
		</UserContext.Provider>
	);
};

export const useUser = () => useContext(UserContext);
