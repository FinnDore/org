"use client";

import { SessionProvider, signIn, signOut, useSession } from "next-auth/react";

export default function Home() {
    return (
        <main className="relative grid min-h-screen place-content-center bg-black text-white">
            <SessionProvider>
                <User />
            </SessionProvider>
        </main>
    );
}

function User() {
    const session = useSession();

    if (session.data) {
        return (
            <div className="flex flex-col gap-2 text-center">
                <img
                    src={session.data.user.image}
                    alt="user"
                    className="rounded-full border border-white/40"
                />
                <p>{session.data.user.name}</p>
                <button onClick={() => signOut()}>Sign out</button>
            </div>
        );
    }

    return (
        <div>
            <button onClick={() => signIn("discord")}>Sign in</button>
        </div>
    );
}
