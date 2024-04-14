"use client";
import { Button } from "@/components/ui/button";
import { Pfp } from "@/components/ui/pfp";
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
            {session.data && !loading && <Pfp user={session.data.user} />}
            {loading && <button>login</button>}
        </div>
    );
}
