"use client";
import { Button } from "@/components/ui/button";
import { useDebounceValue } from "@/lib/utils";
import { api } from "@/trpc/react";
import { signOut, useSession } from "next-auth/react";
import { useRouter } from "next/navigation";

export default function Home() {
    const delteMutation = api.me.deleteAccount.useMutation();
    const router = useRouter();

    return (
        <main className="relative grid min-h-screen place-content-center">
            <div className="flex flex-col justify-center gap-4">
                <User />
                <Button
                    variant="destructive"
                    onClick={async () => {
                        await delteMutation.mutateAsync();
                        await signOut({
                            callbackUrl: "/",
                        });
                    }}
                >
                    Delete account
                </Button>
            </div>
        </main>
    );
}
function User() {
    const session = useSession();

    const loading = useDebounceValue(session.status === "loading", {
        defaultValue: false,
    });
    return (
        <div className="h-32">
            {session.data && !loading && (
                <img
                    src={session.data.user.image!}
                    alt="user"
                    className="h-full rounded-full border border-black/40 dark:border-white/40"
                />
            )}
            {loading && <button>login</button>}
        </div>
    );
}
